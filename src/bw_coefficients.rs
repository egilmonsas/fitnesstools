use crate::{util::poly5, Gender};

/// Compute the wilks coefficient for a lifter, using the original formula
///
/// <https://en.wikipedia.org/wiki/Wilks_coefficient>
///
/// ```
/// use fitnesstools::{wilks, Gender};
/// let wilks_coefficient = wilks(Gender::Male, 80.0);
///
/// assert_eq!(wilks_coefficient, 0.6826985901683169);
/// ```
pub fn wilks(gender: Gender, bodyweight: f64) -> f64 {
    500.0 / poly5(wilks_constants(gender), bodyweight)
}

/// Compute the wilks coefficient for a lifter, using the updated 2020 formula
///
/// <https://en.wikipedia.org/wiki/Wilks_coefficient>
///
/// ```
/// use fitnesstools::{wilks2020, Gender};
/// let wilks_coefficient = wilks2020(Gender::Male, 80.0);
///
/// assert_eq!(wilks_coefficient, 0.8192383082019803);
/// ```
pub fn wilks2020(gender: Gender, bodyweight: f64) -> f64 {
    600.0 / poly5(wilks2020_constants(gender), bodyweight)
}

fn wilks_constants(gender: Gender) -> [f64; 6] {
    match gender {
        Gender::Male => [
            -216.0475144,
            16.2606339,
            -0.002388645,
            -0.00113732,
            7.01863 * 10_f64.powi(-6),
            -1.291 * 10_f64.powi(-8),
        ],
        Gender::Female => [
            594.31747775582,
            -27.23842536447,
            0.82112226871,
            -0.00930733913,
            4.731582 * 10_f64.powi(-5),
            -9.054 * 10_f64.powi(-8),
        ],
    }
}

fn wilks2020_constants(gender: Gender) -> [f64; 6] {
    match gender {
        Gender::Male => [
            47.46178854,
            8.472061379,
            0.07369410346,
            -0.001395833811,
            7.07665973070743 * 10_f64.powi(-6),
            -1.20804336482315 * 10_f64.powi(-8),
        ],
        Gender::Female => [
            -125.4255398,
            13.71219419,
            -0.03307250631,
            -0.001050400051,
            9.38773881462799 * 10_f64.powi(-5),
            -2.3334613884954 * 10_f64.powi(-8),
        ],
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wilks() {
        assert_eq!(wilks(Gender::Male, 80.0), 0.6826985901683169);
    }

    #[test]
    fn test_wilks2020() {
        assert_eq!(wilks2020(Gender::Male, 80.0), 0.8192383082019803);
    }
}
