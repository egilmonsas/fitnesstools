pub fn poly5(coefficients: [f64; 6], x: f64) -> f64 {
    coefficients
        .iter()
        .enumerate()
        .fold(0.0, |acc, (idx, val)| acc + val * x.powi(idx as i32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly5() {
        assert_eq!(poly5([1.0, 0.0, 0.0, 0.0, 0.0, 0.0], 80.0), 1.0);
        assert_eq!(poly5([1.0, 1.0, 0.0, 0.0, 0.0, 0.0], 80.0), 81.0);
        assert_eq!(poly5([1.0, 1.0, 1.0, 0.0, 0.0, 0.0], 80.0), 6481.0);
    }
}
