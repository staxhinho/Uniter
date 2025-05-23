pub fn round(output_raw: f64, decimals: i64) -> f64 {
    let multiplier = 10f64.powi(decimals as i32);
    let rounded = (output_raw * multiplier).round() / multiplier;

    if rounded == 0.0 && output_raw != 0.0 {
        return output_raw;
    }

    rounded
}