use std::f64::consts::PI;

/// Coefficients for the bs2b two-filter crossfeed structure.
///
/// Signal chain per channel:
///   outL = highboost(inL) + lp(inR)
///   outR = highboost(inR) + lp(inL)
///
/// Reference: https://bs2b.sourceforge.net/
/// Canonical presets:
///   Default:   Fc=700 Hz, feed=4.5 dB  (Gd=-6.75, Ad_h=-2.25)
///   Chu Moy:   Fc=700 Hz, feed=6.0 dB  (Gd=-8.0,  Ad_h=-2.0 )
///   Jan Meier: Fc=650 Hz, feed=9.5 dB  (Gd=-10.917, Ad_h=-1.417)
#[derive(Clone, Copy, Default)]
pub struct Bs2bCoefficients {
    /// One-pole LP (crossed path): y[n] = a0*x[n] + b1*y[n-1]
    pub a0: f64,
    pub b1: f64,
    /// First-order IIR highboost (direct path): y[n] = a0_h*x[n] + a1_h*x[n-1] + b1_h*y[n-1]
    pub a0_h: f64,
    pub a1_h: f64,
    pub b1_h: f64,
}

impl Bs2bCoefficients {
    /// Compute from cutoff (Hz), crossfeed level (dB), sample rate.
    ///
    /// Gd/Ad_h split uses the Default preset ratio:
    ///   Gd = -(feed * 1.5)   →  -6.75 dB at feed=4.5
    ///   Ad_h = -(feed * 0.5) →  -2.25 dB at feed=4.5
    /// This matches the bs2b Default preset exactly.
    pub fn compute(fc_hz: f64, feed_db: f64, sample_rate: f64) -> Self {
        let gd = -(feed_db * 1.5);
        let ad_h = -(feed_db * 0.5);

        let g = 10f64.powf(gd / 20.0);
        let a_h = 10f64.powf(ad_h / 20.0);
        let g_h = 1.0 - a_h;

        let gd_h = 20.0 * g_h.ln() / 10f64.ln();
        let fc_h = fc_hz * 2f64.powf((gd - gd_h) / 12.0);

        let x = (-2.0 * PI * fc_hz / sample_rate).exp();
        let x_h = (-2.0 * PI * fc_h / sample_rate).exp();

        Self {
            a0: g * (1.0 - x),
            b1: x,
            a0_h: 1.0 - g_h * (1.0 - x_h),
            a1_h: -x_h,
            b1_h: x_h,
        }
    }
}

/// Per-channel filter state.
/// Each channel holds state for both its LP filter (applied to this channel's signal
/// and mixed into the opposite output) and its highboost filter (applied to this
/// channel's signal for its own output).
pub struct Bs2bChannel {
    pub lp_y1: f64,
    pub hb_x1: f64,
    pub hb_y1: f64,
}

impl Bs2bChannel {
    pub fn new() -> Self {
        Self {
            lp_y1: 0.0,
            hb_x1: 0.0,
            hb_y1: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.lp_y1 = 0.0;
        self.hb_x1 = 0.0;
        self.hb_y1 = 0.0;
    }

    /// One-pole LP: y[n] = a0*x[n] + b1*y[n-1]
    pub fn lp(&mut self, x: f64, c: &Bs2bCoefficients) -> f64 {
        let y = c.a0 * x + c.b1 * self.lp_y1;
        self.lp_y1 = y;
        y
    }

    /// Highboost: y[n] = a0_h*x[n] + a1_h*x[n-1] + b1_h*y[n-1]
    pub fn hb(&mut self, x: f64, c: &Bs2bCoefficients) -> f64 {
        let y = c.a0_h * x + c.a1_h * self.hb_x1 + c.b1_h * self.hb_y1;
        self.hb_x1 = x;
        self.hb_y1 = y;
        y
    }
}

pub struct Bs2b {
    pub left: Bs2bChannel,
    pub right: Bs2bChannel,
    pub coeffs: Bs2bCoefficients,
}

impl Bs2b {
    pub fn new() -> Self {
        Self {
            left: Bs2bChannel::new(),
            right: Bs2bChannel::new(),
            coeffs: Bs2bCoefficients::default(),
        }
    }

    pub fn update_coeffs(&mut self, fc_hz: f64, feed_db: f64, sample_rate: f64) {
        self.coeffs = Bs2bCoefficients::compute(fc_hz, feed_db, sample_rate);
    }

    pub fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
    }

    /// Process one stereo sample pair (no ITD).
    ///   outL = highboost(inL) + lp(inR)
    ///   outR = highboost(inR) + lp(inL)
    #[allow(unused)]
    pub fn process(&mut self, in_l: f64, in_r: f64) -> (f64, f64) {
        self.process_with_itd(in_l, in_r, in_r, in_l)
    }

    /// Process with ITD: direct path uses `in_l`/`in_r`, crossed path uses pre-delayed signals.
    ///   outL = highboost(in_l) + lp(in_r_delayed)
    ///   outR = highboost(in_r) + lp(in_l_delayed)
    pub fn process_with_itd(&mut self, in_l: f64, in_r: f64, in_l_delayed: f64, in_r_delayed: f64) -> (f64, f64) {
        let c = self.coeffs;
        let hb_l = self.left.hb(in_l, &c);
        let lp_r = self.right.lp(in_r_delayed, &c);
        let hb_r = self.right.hb(in_r, &c);
        let lp_l = self.left.lp(in_l_delayed, &c);
        (hb_l + lp_r, hb_r + lp_l)
    }
}
