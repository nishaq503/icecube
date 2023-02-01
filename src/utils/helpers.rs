pub fn check_float(arr: &[f32]) -> Result<(), String> {
    if arr.iter().any(|f| f.is_infinite() || f.is_nan()) {
        Err("All arguments must be non-nan and finite.".to_string())
    } else {
        Ok(())
    }
}

/// Clips a given f32 value to the [-1., 1.] range.
pub fn clip(val: f32) -> f32 {
    if val < -1. {
        -1.
    } else if val > 1. {
        1.
    } else {
        val
    }
}
