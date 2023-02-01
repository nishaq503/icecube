/// Calculate the MAE of the angular distance between two directions.
///
/// Source: https://www.kaggle.com/code/sohier/mean-angular-error
///
/// The two vectors are first converted to cartesian unit vectors, and then
/// their scalar product is computed, which is equal to the cosine of the angle
/// between the two vectors. The inverse cosine (arccos) thereof is then the
/// angle between the two input vectors.
///
/// # Arguments
///
/// * `a_true`: true azimuth values in radians
/// * `z_true`: true zenith values in radians
/// * `a_pred`: predicted azimuth values in radians
/// * `z_pred`: predicted zenith values in radians
///
/// # Returns
///     mean over the angular distances in radian
pub fn angular_dist_score(
    a_true: &[f32],
    z_true: &[f32],
    a_pred: &[f32],
    z_pred: &[f32],
) -> Result<f32, String> {
    crate::utils::check_float(a_true)?;
    crate::utils::check_float(z_true)?;
    crate::utils::check_float(a_pred)?;
    crate::utils::check_float(z_pred)?;

    Ok(a_true
        .iter()
        .map(|&f| libm::sinf(f))
        .zip(a_true.iter().map(|&f| libm::cosf(f)))
        .zip(z_true.iter().map(|&f| libm::sinf(f)))
        .zip(z_true.iter().map(|&f| libm::cosf(f)))
        .zip(a_pred.iter().map(|&f| libm::sinf(f)))
        .zip(a_pred.iter().map(|&f| libm::cosf(f)))
        .zip(z_pred.iter().map(|&f| libm::sinf(f)))
        .zip(z_pred.iter().map(|&f| libm::cosf(f)))
        .map(|(((((((sa1, ca1), sz1), cz1), sa2), ca2), sz2), cz2)| {
            sz1 * sz2 * (ca1 * ca2 + sa1 * sa2) + (cz1 * cz2)
        })
        .map(crate::utils::clip)
        .map(libm::acosf)
        .map(f32::abs)
        .sum::<f32>()
        / a_true.len() as f32)
}
