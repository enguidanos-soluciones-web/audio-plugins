#pragma once

#include <algorithm>
#include <cmath>
#include <mutex>
#include <vector>

// Mono Lanczos-A windowed sinc stream resampler.
// Push input samples at input_sr, pull output samples at output_sr.
// No external dependencies beyond the C++ standard library.
class LanczosResampler {
public:
    static constexpr int A   = 12;    // filter half-width (taps = 2*A per output sample)
    static constexpr int RES = 8192;  // filter table resolution per input-sample unit
    static constexpr int BUF = 65536; // ring buffer size; must be a power of 2
    static_assert((BUF & (BUF - 1)) == 0, "BUF must be a power of 2");

    LanczosResampler(double input_sr, double output_sr)
        : mRatio(output_sr / input_sr)
        , mBuf(BUF, 0.0)
    {
        ensure_table();
        mWritePos = A; // pre-advance so history exists for the first output sample
        mPhase    = 0.0;
    }

    // Feed n samples. Returns number of output samples now available.
    int push(const double* input, int n) {
        for (int i = 0; i < n; ++i) {
            mBuf[mWritePos & (BUF - 1)] = input[i];
            ++mWritePos;
        }
        return available();
    }

    // Pull up to n samples at output_sr. Returns how many were written.
    int pull(double* output, int n) {
        int count = 0;
        while (count < n && mPhase + A <= static_cast<double>(mWritePos)) {
            output[count++] = read_at(mPhase);
            mPhase += 1.0 / mRatio;
        }
        return count;
    }

    int available() const {
        double limit = static_cast<double>(mWritePos) - A;
        if (limit <= mPhase) return 0;
        return static_cast<int>((limit - mPhase) * mRatio);
    }

    void clear() {
        std::fill(mBuf.begin(), mBuf.end(), 0.0);
        mWritePos = A;
        mPhase    = 0.0;
    }

private:
    double            mRatio;
    std::vector<double> mBuf;
    long long         mWritePos = 0;
    double            mPhase    = 0.0;

    static constexpr double kPI = 3.14159265358979323846;

    static double kernel(double x) {
        if (x < 1e-8) return 1.0;
        if (x >= A)   return 0.0;
        const double px = kPI * x;
        return static_cast<double>(A) * std::sin(px) * std::sin(px / A) / (px * px);
    }

    static double* table() {
        static double t[A * RES + 1];
        return t;
    }

    static void ensure_table() {
        static std::once_flag flag;
        std::call_once(flag, [] {
            double* t = table();
            for (int i = 0; i <= A * RES; ++i)
                t[i] = kernel(static_cast<double>(i) / RES);
        });
    }

    double lookup(double x) const {
        const double* t  = table();
        const double  fi = x * RES;
        const int     i  = static_cast<int>(fi);
        if (i >= A * RES) return 0.0;
        const double frac = fi - i;
        return t[i] * (1.0 - frac) + t[i + 1] * frac;
    }

    double read_at(double pos) const {
        const long long i0   = static_cast<long long>(std::floor(pos));
        const double    frac = pos - static_cast<double>(i0);
        double          sum  = 0.0;
        for (int k = -A + 1; k <= A; ++k) {
            const double w = lookup(std::abs(static_cast<double>(k) - frac));
            if (w != 0.0)
                sum += mBuf[(i0 + k) & (BUF - 1)] * w;
        }
        return sum;
    }
};
