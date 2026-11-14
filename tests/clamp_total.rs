//! `f32::clamp` panics when `min > max` or either is NaN. Every clamp in the crate whose bounds are
//! computed must be total: a one-ulp inversion of two bounds that are equal on paper, or a NaN bound,
//! is a value, never a panic (G-91, #102). Deterministic: the inputs are enumerated, not random.

use bevy_gooey::{pin_to_edge, slider_text, split_clamp, split_mins, step_value};

/// Extents and minimums whose two minimums sum to the extent (or a little more), over a spread of
/// magnitudes: the shape `split_mins` scales.
fn pairs() -> Vec<(f32, f32, f32)> {
    let mut out = Vec::new();
    for extent in (1..=400).map(|i| i as f32 * 1.7 + 0.137) {
        for share in [0.1, 0.25, 0.3, 0.37, 0.5, 0.6563119, 0.77, 0.9] {
            let first = extent * share;
            for over in [0.0, 1e-4, 1.0, 40.0] {
                out.push((extent, first, extent - first + over));
            }
        }
    }
    out
}

/// The bounds `split_clamp` computes really do invert by an ulp for some of these inputs: the test
/// below is not vacuous.
#[test]
fn the_split_bounds_really_invert_by_an_ulp_for_some_extents() {
    let inverted = pairs()
        .into_iter()
        .filter(|&(extent, a, b)| {
            let (first, second) = split_mins(extent, a, b);
            first / extent > 1.0 - second / extent
        })
        .count();
    assert!(inverted > 0, "no input reproduced min > max");
}

#[test]
fn a_split_whose_minimums_fill_the_extent_never_panics_and_stays_sane() {
    for (extent, a, b) in pairs() {
        for fraction in [
            -1.0,
            0.0,
            0.2,
            0.5,
            0.6563119,
            0.9,
            1.0,
            7.0,
            f32::NAN,
            f32::INFINITY,
        ] {
            let got = split_clamp(fraction, extent, a, b);
            assert!(
                got.is_finite() && (0.0..=1.0).contains(&got),
                "{got} for {fraction} {extent} {a} {b}"
            );
            if extent > 0.0 {
                let (first, second) = split_mins(extent, a, b);
                let slack = 4.0 * f32::EPSILON;
                assert!(
                    got >= first / extent - slack && got <= 1.0 - second / extent + slack,
                    "{got} outside the minimums ({extent} {a} {b})"
                );
            }
        }
    }
}

/// The reported pair: `min = 0.6563119`, `max = 0.65631187`, found by enumeration and held to a point
/// between them.
#[test]
fn the_reported_ulp_inversion_is_held_to_the_point_between_the_bounds() {
    let hit = pairs().into_iter().find_map(|(extent, a, b)| {
        let (first, second) = split_mins(extent, a, b);
        (first / extent > 1.0 - second / extent).then_some((
            extent,
            a,
            b,
            first / extent,
            1.0 - second / extent,
        ))
    });
    let (extent, a, b, lo, hi) = hit.expect("an inverted pair");
    for fraction in [0.0, 0.5, 1.0] {
        let got = split_clamp(fraction, extent, a, b);
        assert!(got >= hi && got <= lo, "{got} not in [{hi}, {lo}]");
    }
}

#[test]
fn a_nan_or_inverted_hold_bound_is_a_value_not_a_panic() {
    // `hold` is the clamp behind the slider and number steps.
    assert_eq!(step_value(0.3, 1.0, 0.1, 0.0, 1.0), 0.4);
    assert_eq!(
        step_value(0.3, 1.0, 0.1, 1.0, 0.0),
        0.4,
        "bounds in either order"
    );
    let got = step_value(0.3, 1.0, 0.1, f32::NAN, f32::NAN);
    assert!(
        (got - 0.4).abs() < 1e-6,
        "no bounds to hold to: the stepped value, {got}"
    );
    assert_eq!(
        step_value(0.3, 1.0, 0.1, f32::NAN, 0.35),
        0.35,
        "one bound is still a bound"
    );
    assert!(!slider_text(0.5, f32::NAN, f32::NAN).is_empty());
}

#[test]
fn a_nan_edge_of_the_pin_rectangle_gives_the_centre_and_never_panics() {
    let centre = (640.0, 360.0);
    for rect in [
        ((f32::NAN, 36.0), (1216.0, 684.0)),
        ((64.0, f32::NAN), (1216.0, 684.0)),
        ((64.0, 36.0), (f32::NAN, 684.0)),
        ((64.0, 36.0), (1216.0, f32::NAN)),
        ((f32::NAN, f32::NAN), (f32::NAN, f32::NAN)),
    ] {
        assert_eq!(
            pin_to_edge((5000.0, 5000.0), centre, rect.0, rect.1),
            centre,
            "{rect:?}"
        );
    }
    assert_eq!(
        pin_to_edge((5000.0, 360.0), centre, (64.0, 36.0), (1216.0, 684.0)),
        (1216.0, 360.0)
    );
}
