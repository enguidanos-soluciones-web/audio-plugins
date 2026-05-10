/// Pink noise generator using Paul Kellett's refined 6-pole algorithm.
///
/// White noise (xorshift64) is filtered through six one-pole IIR filters
/// whose combined response closely approximates a -3 dB/octave (1/f) power
/// spectrum across the full audible range.
///
/// The 6-pole version is required for calibration accuracy: the 3-pole
/// approximation has significant low-frequency ripple (<100 Hz) which
/// contaminates ITD perception (dominant below 1.5 kHz) and the crossfeed LP
/// region (300–2000 Hz).
///
/// Coefficients: Paul Kellett, music-dsp mailing list, c. 2002.
/// Reference: https://www.firstpr.com.au/dsp/pink-noise/
///
/// Zero allocations, no external dependencies.
pub struct PinkNoise {
    // xorshift64 PRNG state (must be non-zero)
    rng: u64,
    // 6-pole IIR filter states
    b0: f64,
    b1: f64,
    b2: f64,
    b3: f64,
    b4: f64,
    b5: f64,
}

impl PinkNoise {
    pub fn new() -> Self {
        Self {
            rng: 0xdeadbeefcafe1337, // non-zero seed
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            b3: 0.0,
            b4: 0.0,
            b5: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.b0 = 0.0;
        self.b1 = 0.0;
        self.b2 = 0.0;
        self.b3 = 0.0;
        self.b4 = 0.0;
        self.b5 = 0.0;
        // Keep rng state — resetting it would produce the same sequence on every reset
    }

    /// Generate one sample of pink noise, scaled to -12 dBFS RMS target.
    #[inline]
    pub fn next(&mut self) -> f64 {
        let white = self.white();
        self.b0 =  0.99886 * self.b0 + white * 0.0555179;
        self.b1 =  0.99332 * self.b1 + white * 0.0750759;
        self.b2 =  0.96900 * self.b2 + white * 0.1538520;
        self.b3 =  0.86650 * self.b3 + white * 0.3104856;
        self.b4 =  0.55000 * self.b4 + white * 0.5329522;
        self.b5 = -0.76160 * self.b5 - white * 0.0168980;
        let pink = (self.b0 + self.b1 + self.b2 + self.b3 + self.b4 + self.b5 + white * 0.5362) * 0.11;
        pink.clamp(-1.0, 1.0)
    }

    /// xorshift64 → uniform f64 in [-1, 1].
    #[inline]
    fn white(&mut self) -> f64 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        // Map u64 to [-1, 1]
        (self.rng as i64 as f64) / (i64::MAX as f64)
    }
}
