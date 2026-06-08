#pragma once

#include "dsp.h"
#include "resampler.h"
#include "rust/cxx.h"
#include <cmath>
#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <vector>

class ResamplingNAM {
public:
    static constexpr double kFallbackSampleRate = 48000.0;

    explicit ResamplingNAM(std::unique_ptr<nam::DSP> model)
        : mModel(std::move(model))
        , mModelSampleRate(model_sr(*mModel))
    {}

    void process(double** input, double** output, int n) {
        if (!need_resample()) {
            mModel->process(input, output, n);
            return;
        }

        // 1. Feed session-SR input into the upsampler (session → model SR).
        int avail = mInputResampler->push(input[0], n);
        avail = std::min(avail, static_cast<int>(mModelInBuf.size()));

        if (avail > 0) {
            mInputResampler->pull(mModelInBuf.data(), avail);

            // 2. Process with NAM at its native sample rate.
            double* in_ptr  = mModelInBuf.data();
            double* out_ptr = mModelOutBuf.data();
            mModel->process(&in_ptr, &out_ptr, avail);

            // 3. Feed model-SR output into the downsampler (model SR → session).
            mOutputResampler->push(mModelOutBuf.data(), avail);
        }

        // 4. Pull session-SR output; zero-fill during startup latency.
        int pulled = mOutputResampler->pull(output[0], n);
        for (int i = pulled; i < n; ++i) output[0][i] = 0.0;
    }

    void Reset(double session_sr, int max_block_size) {
        mSessionSampleRate = session_sr;
        if (need_resample()) {
            mInputResampler.emplace(session_sr, mModelSampleRate);
            mOutputResampler.emplace(mModelSampleRate, session_sr);
            const int inner_max = static_cast<int>(
                std::ceil(static_cast<double>(max_block_size) * mModelSampleRate / session_sr))
                + LanczosResampler::A * 4;
            mModelInBuf.resize(inner_max);
            mModelOutBuf.resize(inner_max);
            mModel->ResetAndPrewarm(mModelSampleRate, inner_max);
        } else {
            mInputResampler.reset();
            mOutputResampler.reset();
            mModel->ResetAndPrewarm(session_sr, max_block_size);
        }
    }

    void ResetAndPrewarm(double session_sr, int max_block_size) {
        Reset(session_sr, max_block_size);
    }

    bool HasLoudness()          const { return mModel->HasLoudness(); }
    double GetLoudness()        const { return mModel->GetLoudness(); }
    double GetModelSampleRate() const { return mModelSampleRate; }

private:
    bool need_resample() const { return mSessionSampleRate != mModelSampleRate; }

    static double model_sr(const nam::DSP& m) {
        const double sr = m.GetExpectedSampleRate();
        return sr > 0.0 ? sr : kFallbackSampleRate;
    }

    std::unique_ptr<nam::DSP>         mModel;
    double                            mModelSampleRate;
    double                            mSessionSampleRate = 0.0;
    std::optional<LanczosResampler>   mInputResampler;
    std::optional<LanczosResampler>   mOutputResampler;
    std::vector<double>               mModelInBuf;
    std::vector<double>               mModelOutBuf;
};

struct NamDsp {
    std::unique_ptr<ResamplingNAM> inner;
};

std::unique_ptr<NamDsp> load(rust::Str json);

void process(NamDsp& dsp, rust::Slice<const double> input,
             rust::Slice<double> output);
void reset(NamDsp& dsp, double sample_rate, int32_t max_block_size);
void reset_and_prewarm(NamDsp& dsp, double sample_rate, int32_t max_block_size);
bool has_loudness(const NamDsp& dsp);
double get_loudness(const NamDsp& dsp);
double get_sample_rate_from_nam_file(rust::Str json);
int32_t check_nam_version_support(rust::Str json);
