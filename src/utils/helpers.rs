pub fn check_float(arr: &[f32]) -> Result<(), String> {
    if arr.iter().any(|f| f.is_infinite() || f.is_nan()) {
        Err("All arguments must be non-nan and finite.".to_string())
    } else {
        Ok(())
    }
}
