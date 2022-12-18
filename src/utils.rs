use crate::types::Point;

pub(crate) fn polar_to_cartesian(c: Point, radius: f32, angle_degrees: f32) -> Point {
    let angle_radians = (angle_degrees - 90.0).to_radians();
    Point {
        x: c.x + radius * angle_radians.cos(),
        y: c.y + radius * angle_radians.sin(),
    }
}

pub(crate) fn normalize_series(series: &[f32]) -> Vec<f32> {
    let r = series.iter().copied().reduce(f32::max).unwrap() / 100.0;
    series.iter().map(|v| v / r).collect()
}

pub(crate) fn magnitude(value: f32) -> f32 {
    10.0_f32.powf(value.abs().log10().floor())
}
