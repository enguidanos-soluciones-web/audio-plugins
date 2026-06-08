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
    dsp::itd::ItdDelay,
    parameters::{
        Parameter, Range, Select, angle::Angle, bs2b_low_shelf::Bs2bLowShelf, calibration_mode::CalibrationMode, center::Center,
        cutoff::Cutoff, gain::Gain, lrswap::LRSwap, phase::Phase, solo::Solo, xfeed::XFeed, xfeed_slope::XFeedSlope,
    },
    state::AudioThreadState,
    utils::{
        decibel_conversion::DecibelConversion,
        tuples::{CopyFillFromLeft, CopyFillFromRight, Reverse},
    },
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
    let xfeed_slope = snapshot.values[Parameter::<XFeedSlope, Range>::ID];
    let bs2b_low_shelf = snapshot.values[Parameter::<Bs2bLowShelf, Range>::ID];
    let angle = snapshot.values[Parameter::<Angle, Range>::ID];
    let calibration_mode = snapshot.values[Parameter::<CalibrationMode, Select>::ID];
    let center_gain = snapshot.values[Parameter::<Center, Range>::ID];
    let lrswap = LRSwap::from(snapshot.values[Parameter::<LRSwap, Select>::ID]);
    let solo = Solo::from(snapshot.values[Parameter::<Solo, Select>::ID]);
    let phase = Phase::from(snapshot.values[Parameter::<Phase, Select>::ID]);
    let makeup_gain = snapshot.values[Parameter::<Gain, Range>::ID];

    let in_l = unsafe { std::slice::from_raw_parts(in_l, nframes) };
    let in_r = unsafe { std::slice::from_raw_parts(in_r, nframes) };
    let out_l = unsafe { std::slice::from_raw_parts_mut(out_l, nframes) };
    let out_r = unsafe { std::slice::from_raw_parts_mut(out_r, nframes) };

    for i in 0..nframes {
        let calibration_output = match calibration_mode.round() as u8 {
            // Calibration off — pass incoming audio through bs2b
            CalibrationMode::OFF => (in_l[i], in_r[i]),
            // L-only pink noise at -12 dBFS — direct signal in left ear, crossed LP in right ear.
            // Adjust Cutoff: controls which frequencies appear in the right (crossed) ear.
            // Adjust XFeed: controls how loud the crossed signal is in the right ear.
            CalibrationMode::CONTINUOUS => (audio_thread.dsp.calibration.pink_noise.next(), 0.0),
            // Intermittent alternating mono pink noise at -12 dBFS — 500ms L, then 500ms R, repeat.
            // Good for tuning Angle: hear how the ITD delay externalizes the image.
            CalibrationMode::INTERMITTENT => {
                audio_thread.dsp.calibration.phase =
                    (audio_thread.dsp.calibration.phase + 1) % (audio_thread.dsp.calibration.half_period * 2);

                let sample = audio_thread.dsp.calibration.pink_noise.next();

                if audio_thread.dsp.calibration.phase < audio_thread.dsp.calibration.half_period {
                    (sample, 0.0)
                } else {
                    (0.0, sample)
                }
            }

            _ => (in_l[i], in_r[i]),
        };

        // 1. L/R swap
        let mut lrswap_output = calibration_output;
        if matches!(lrswap, LRSwap::On) {
            lrswap_output = calibration_output.reverse()
        };

        // 2. Phase inversion
        let mut phase_inversion_output = lrswap_output;
        if matches!(phase, Phase::L) {
            phase_inversion_output.0 = -phase_inversion_output.0;
        };
        if matches!(phase, Phase::R) {
            phase_inversion_output.1 = -phase_inversion_output.1;
        };

        // 3. ITD delay for the crossed path
        let itd_delayed_output = audio_thread.dsp.itd.process(
            phase_inversion_output,
            ItdDelay::angle_to_delay_samples(angle, audio_thread.sample_rate),
        );

        // 4. bs2b
        audio_thread
            .dsp
            .bs2b
            .update_coeffs(cutoff, xfeed, xfeed_slope, bs2b_low_shelf, audio_thread.sample_rate);
        let bs2b_output = audio_thread.dsp.bs2b.process_with_itd(phase_inversion_output, itd_delayed_output);

        // 5. M/S center attenuation (SPL Phonitor 3 Center knob)
        let mid = (bs2b_output.0 + bs2b_output.1) * 0.5;
        let side = (bs2b_output.0 - bs2b_output.1) * 0.5;
        let mid_gain = DecibelConversion::Amplitude.to_linear(center_gain);
        let center_attenuated_output = (mid * mid_gain + side, mid * mid_gain - side);

        // 6. Solo (post-matrix, copy processed channel to both ears)
        let mut solo_output = center_attenuated_output;
        if matches!(solo, Solo::L) {
            solo_output = center_attenuated_output.copy_fill_from_left();
        };
        if matches!(solo, Solo::R) {
            solo_output = center_attenuated_output.copy_fill_from_right();
        };

        // 7. Makeup gain (compensates level loss from bs2b/center processing)
        out_l[i] = solo_output.0 * DecibelConversion::Amplitude.to_linear(makeup_gain);
        out_r[i] = solo_output.1 * DecibelConversion::Amplitude.to_linear(makeup_gain);
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
    let xfeed_slope = snapshot.values[Parameter::<XFeedSlope, Range>::ID];
    let bs2b_low_shelf = snapshot.values[Parameter::<Bs2bLowShelf, Range>::ID];
    let angle = snapshot.values[Parameter::<Angle, Range>::ID];
    let calibration_mode = snapshot.values[Parameter::<CalibrationMode, Select>::ID];
    let center_gain = snapshot.values[Parameter::<Center, Range>::ID];
    let lrswap = LRSwap::from(snapshot.values[Parameter::<LRSwap, Select>::ID]);
    let solo = Solo::from(snapshot.values[Parameter::<Solo, Select>::ID]);
    let phase = Phase::from(snapshot.values[Parameter::<Phase, Select>::ID]);
    let makeup_gain = snapshot.values[Parameter::<Gain, Range>::ID];

    let in_l = unsafe { std::slice::from_raw_parts(in_l, nframes) };
    let in_r = unsafe { std::slice::from_raw_parts(in_r, nframes) };
    let out_l = unsafe { std::slice::from_raw_parts_mut(out_l, nframes) };
    let out_r = unsafe { std::slice::from_raw_parts_mut(out_r, nframes) };

    for i in 0..nframes {
        let calibration_output = match calibration_mode.round() as u8 {
            // Calibration off — pass incoming audio through bs2b
            CalibrationMode::OFF => (in_l[i] as f64, in_r[i] as f64),
            // L-only pink noise at -12 dBFS — direct signal in left ear, crossed LP in right ear.
            // Adjust Cutoff: controls which frequencies appear in the right (crossed) ear.
            // Adjust XFeed: controls how loud the crossed signal is in the right ear.
            CalibrationMode::CONTINUOUS => (audio_thread.dsp.calibration.pink_noise.next(), 0.0),
            // Intermittent alternating mono pink noise at -12 dBFS — 500ms L, then 500ms R, repeat.
            // Good for tuning Angle: hear how the ITD delay externalizes the image.
            CalibrationMode::INTERMITTENT => {
                audio_thread.dsp.calibration.phase =
                    (audio_thread.dsp.calibration.phase + 1) % (audio_thread.dsp.calibration.half_period * 2);

                let sample = audio_thread.dsp.calibration.pink_noise.next();

                if audio_thread.dsp.calibration.phase < audio_thread.dsp.calibration.half_period {
                    (sample, 0.0)
                } else {
                    (0.0, sample)
                }
            }
            _ => (in_l[i] as f64, in_r[i] as f64),
        };

        // 1. L/R swap
        let mut lrswap_output = calibration_output;
        if matches!(lrswap, LRSwap::On) {
            lrswap_output = calibration_output.reverse()
        };

        // 2. Phase inversion
        let mut phase_inversion_output = lrswap_output;
        if matches!(phase, Phase::L) {
            phase_inversion_output.0 = -phase_inversion_output.0;
        };
        if matches!(phase, Phase::R) {
            phase_inversion_output.1 = -phase_inversion_output.1;
        };

        // 3. ITD delay for the crossed path
        let itd_delayed_output = audio_thread.dsp.itd.process(
            phase_inversion_output,
            ItdDelay::angle_to_delay_samples(angle, audio_thread.sample_rate),
        );

        // 4. bs2b
        audio_thread
            .dsp
            .bs2b
            .update_coeffs(cutoff, xfeed, xfeed_slope, bs2b_low_shelf, audio_thread.sample_rate);
        let bs2b_output = audio_thread.dsp.bs2b.process_with_itd(phase_inversion_output, itd_delayed_output);

        // 5. M/S center attenuation (SPL Phonitor 3 Center knob)
        let mid = (bs2b_output.0 + bs2b_output.1) * 0.5;
        let side = (bs2b_output.0 - bs2b_output.1) * 0.5;
        let mid_gain = DecibelConversion::Amplitude.to_linear(center_gain);
        let center_attenuated_output = (mid * mid_gain + side, mid * mid_gain - side);

        // 6. Solo (post-matrix, copy processed channel to both ears)
        let mut solo_output = center_attenuated_output;
        if matches!(solo, Solo::L) {
            solo_output = center_attenuated_output.copy_fill_from_left();
        };
        if matches!(solo, Solo::R) {
            solo_output = center_attenuated_output.copy_fill_from_right();
        };

        // 7. Makeup gain (compensates level loss from bs2b/center processing)
        out_l[i] = (solo_output.0 * DecibelConversion::Amplitude.to_linear(makeup_gain)) as f32;
        out_r[i] = (solo_output.1 * DecibelConversion::Amplitude.to_linear(makeup_gain)) as f32;
    }
}
