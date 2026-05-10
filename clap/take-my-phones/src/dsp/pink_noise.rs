/// Pink noise generator using Paul Kellet's 3-pole approximation.
///
/// White noise (xorshift64) is filtered through three one-pole IIR filters
/// whose combined response approximates a -3dB/octave (1/f) power spectrum.
///
/// Zero allocations, no external dependencies.
pub struct PinkNoise {
    // xorshift64 PRNG state (must be non-zero)
    rng: u64,
    // 3-pole IIR filter states (Kellet method)
    b0: f64,
    b1: f64,
    b2: f64,
}

impl PinkNoise {
    pub fn new() -> Self {
        Self {
            rng: 0xdeadbeefcafe1337, // non-zero seed
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.b0 = 0.0;
        self.b1 = 0.0;
        self.b2 = 0.0;
        // Keep rng state — resetting it would produce the same sequence on every reset
    }

    /// Generate one sample of pink noise in [-1, 1].
    #[inline]
    pub fn next(&mut self) -> f64 {
        let white = self.white();
        self.b0 = 0.99886 * self.b0 + white * 0.0555179;
        self.b1 = 0.99332 * self.b1 + white * 0.0750759;
        self.b2 = 0.96900 * self.b2 + white * 0.1538520;
        let pink = (self.b0 + self.b1 + self.b2 + white * 0.5362) * 0.11;
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
