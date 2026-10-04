//! Display-only observation modes and true-value ranges for monitor traces.
//!
//! Values use the caller's engineering units. None of these operations change
//! samples, integration, or the recording state.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ChartObservationMode {
    #[default]
    Overview,
    Detail,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum RangeMode {
    #[default]
    Local,
    FromZero,
    Locked,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TraceRange {
    pub(crate) minimum: f64,
    pub(crate) maximum: f64,
}

impl TraceRange {
    /// An overview range containing zero and every finite raw sample.
    ///
    /// Each nonzero bound is rounded outwards to a 1/2/5 engineering step.
    /// Signed data retains its sign rather than folding negative values to zero.
    pub(crate) fn from_zero(points: &[[f64; 2]]) -> Self {
        let Some((minimum, maximum)) = finite_bounds(points) else {
            return Self {
                minimum: 0.0,
                maximum: 1.0,
            };
        };
        let minimum = if minimum < 0.0 {
            -nice_ceiling_with_margin(-minimum)
        } else {
            0.0
        };
        let maximum = if maximum > 0.0 {
            nice_ceiling_with_margin(maximum)
        } else {
            0.0
        };
        if minimum == maximum {
            Self {
                minimum: 0.0,
                maximum: 1.0,
            }
        } else {
            Self { minimum, maximum }
        }
    }

    /// A local range based on raw extrema, with 8% padding on either side.
    ///
    /// `min_span` is a caller-selected resolution floor in the same units as
    /// the values. Flat signals remain centered on their measured baseline;
    /// empty or invalid signals receive a finite range centered on zero.
    pub(crate) fn local(points: &[[f64; 2]], min_span: f64) -> Self {
        let min_span = if min_span.is_finite() && min_span > 0.0 {
            min_span
        } else {
            1e-6
        };
        let (minimum, maximum) = finite_bounds(points).unwrap_or((0.0, 0.0));
        let observed_span = maximum - minimum;
        // Keep a range representable even if the caller's resolution floor is
        // smaller than floating-point spacing at the measured baseline.
        let representation_floor = minimum.abs().max(maximum.abs()) * f64::EPSILON * 8.0;
        let span = observed_span.max(min_span).max(representation_floor);
        if !span.is_finite() {
            return Self { minimum, maximum };
        }
        let center = minimum + observed_span * 0.5;
        let half_span = span * 0.58;
        Self {
            minimum: (center - half_span).max(-f64::MAX).min(minimum),
            maximum: (center + half_span).min(f64::MAX).max(maximum),
        }
    }

    pub(crate) fn span(self) -> f64 {
        self.maximum - self.minimum
    }

    /// Map true values to plot coordinates without clipping out-of-range data.
    pub(crate) fn normalize(self, value: f64) -> f64 {
        let span = self.span();
        if span.is_infinite() {
            // Halving first keeps extreme finite signed ranges usable.
            (value * 0.5 - self.minimum * 0.5) / (self.maximum * 0.5 - self.minimum * 0.5)
        } else {
            (value - self.minimum) / span
        }
    }

    pub(crate) fn denormalize(self, value: f64) -> f64 {
        // Weighted endpoints avoid an overflowing span for extreme ranges.
        if self.span().is_infinite() {
            self.minimum * (1.0 - value) + self.maximum * value
        } else {
            self.minimum + value * self.span()
        }
    }

    pub(crate) fn contains(self, value: f64) -> bool {
        value.is_finite() && value >= self.minimum && value <= self.maximum
    }
}

fn finite_bounds(points: &[[f64; 2]]) -> Option<(f64, f64)> {
    points
        .iter()
        .filter(|point| point[0].is_finite() && point[1].is_finite())
        .map(|point| point[1])
        .fold(None, |bounds, value| match bounds {
            None => Some((value, value)),
            Some((minimum, maximum)) => Some((minimum.min(value), maximum.max(value))),
        })
}

fn nice_ceiling_with_margin(value: f64) -> f64 {
    let padded = value * 1.06;
    if !padded.is_finite() {
        return f64::MAX;
    }
    let magnitude = 10_f64.powf(padded.log10().floor());
    if magnitude == 0.0 {
        return padded;
    }
    let normalized = padded / magnitude;
    let step = if normalized <= 1.0 {
        1.0
    } else if normalized <= 2.0 {
        2.0
    } else if normalized <= 5.0 {
        5.0
    } else {
        10.0
    };
    (step * magnitude).min(f64::MAX).max(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
    }

    #[test]
    fn local_voltage_exposes_thirty_millivolts_above_nine_volts() {
        let points = [[0.0, 9.0], [1.0, 9.03]];
        let local = TraceRange::local(&points, 0.001);
        let overview = TraceRange::from_zero(&points);
        close(local.minimum, 8.9976);
        close(local.maximum, 9.0324);
        close(overview.minimum, 0.0);
        close(overview.maximum, 10.0);
        assert!(local.normalize(9.03) - local.normalize(9.0) > 0.8);
        assert!(overview.normalize(9.03) - overview.normalize(9.0) < 0.004);
    }

    #[test]
    fn local_microamp_signal_does_not_require_zero_in_the_range() {
        let points = [[0.0, 5e-6], [1.0, 7e-6]];
        let range = TraceRange::local(&points, 1e-7);
        close(range.minimum, 4.84e-6);
        close(range.maximum, 7.16e-6);
        assert!(!range.contains(0.0));
        assert!(range.contains(5e-6));
        assert!(range.contains(7e-6));
    }

    #[test]
    fn both_ranges_preserve_raw_spikes_and_signed_values() {
        let points = [[0.0, 0.1], [1.0, 4.2], [2.0, -1.4], [3.0, 0.1]];
        let overview = TraceRange::from_zero(&points);
        close(overview.minimum, -2.0);
        close(overview.maximum, 5.0);
        for range in [overview, TraceRange::local(&points, 0.001)] {
            for [_, value] in points {
                assert!(range.contains(value));
                close(range.denormalize(range.normalize(value)), value);
            }
        }
    }

    #[test]
    fn constant_signal_respects_resolution_and_keeps_its_baseline() {
        let range = TraceRange::local(&[[0.0, 9.0], [1.0, 9.0]], 0.01);
        close(range.span(), 0.0116);
        close(range.denormalize(0.5), 9.0);
        assert!(range.contains(9.0));
        assert!(!range.contains(0.0));
    }

    #[test]
    fn invalid_samples_and_empty_signals_have_a_finite_fallback() {
        let invalid = [[0.0, f64::NAN], [1.0, f64::INFINITY], [f64::NAN, 99.0]];
        for points in [&[][..], &invalid[..]] {
            let overview = TraceRange::from_zero(points);
            close(overview.minimum, 0.0);
            close(overview.maximum, 1.0);
            for floor in [0.01, 0.0, -1.0, f64::NAN, f64::INFINITY] {
                let range = TraceRange::local(points, floor);
                assert!(range.minimum.is_finite());
                assert!(range.maximum.is_finite());
                assert!(range.span() > 0.0);
                close(range.normalize(0.0), 0.5);
            }
        }
        let zero = TraceRange::from_zero(&[[0.0, 0.0]]);
        close(zero.span(), 1.0);
    }

    #[test]
    fn locked_range_maps_outside_values_without_clamping() {
        let range = TraceRange {
            minimum: 8.95,
            maximum: 9.05,
        };
        assert!(range.normalize(9.1) > 1.0);
        assert!(range.normalize(8.9) < 0.0);
        assert!(!range.contains(9.1));
        assert!(!range.contains(f64::NAN));
        close(range.denormalize(range.normalize(9.1)), 9.1);
        close(range.denormalize(range.normalize(8.9)), 8.9);
    }

    #[test]
    fn extreme_finite_values_are_not_replaced_by_small_fallbacks() {
        let points = [[0.0, -f64::MAX], [1.0, f64::MAX]];
        for range in [TraceRange::from_zero(&points), TraceRange::local(&points, 0.001)] {
            assert!(range.contains(-f64::MAX));
            assert!(range.contains(f64::MAX));
            close(range.normalize(0.0), 0.5);
            close(range.denormalize(0.5), 0.0);
        }
    }
}
