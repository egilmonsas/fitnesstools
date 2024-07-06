pub enum E1rmFormula {
    Epley,
    Brzycki,
    Adams,
    Baechle,
    ALL,
}

impl E1rmFormula {
    fn e1rm(&self, reps: f64) -> f64 {
        match self {
            E1rmFormula::Epley => Self::e1rm_epley(reps),
            E1rmFormula::Brzycki => Self::e1rm_brzycki(reps),
            E1rmFormula::Adams => Self::e1rm_adams(reps),
            E1rmFormula::Baechle => Self::e1rm_baechle(reps),
            E1rmFormula::ALL => Self::e1rm_all(reps),
        }
    }
    fn e1rm_all(reps: f64) -> f64 {
        (Self::e1rm_epley(reps)
            + Self::e1rm_brzycki(reps)
            + Self::e1rm_adams(reps)
            + Self::e1rm_baechle(reps))
            / 4.0
    }
    fn e1rm_epley(reps: f64) -> f64 {
        1.0 + reps / 30.0
    }
    fn e1rm_brzycki(reps: f64) -> f64 {
        36.0 / (37.0 - reps)
    }
    fn e1rm_adams(reps: f64) -> f64 {
        1.0 / (1.0 - 0.02 * reps)
    }
    fn e1rm_baechle(reps: f64) -> f64 {
        1.0 + 0.033 * reps
    }

    fn maxreps(&self, fraction_of_e1rm: f64) -> f64 {
        match self {
            E1rmFormula::Epley => Self::maxreps_epley(fraction_of_e1rm),
            E1rmFormula::Brzycki => Self::maxreps_brzycki(fraction_of_e1rm),
            E1rmFormula::Adams => Self::maxreps_adams(fraction_of_e1rm),
            E1rmFormula::Baechle => Self::maxreps_baechle(fraction_of_e1rm),
            E1rmFormula::ALL => Self::maxreps_all(fraction_of_e1rm),
        }
    }
    fn maxreps_all(fraction_of_e1rm: f64) -> f64 {
        (Self::maxreps_epley(fraction_of_e1rm)
            + Self::maxreps_brzycki(fraction_of_e1rm)
            + Self::maxreps_adams(fraction_of_e1rm)
            + Self::maxreps_baechle(fraction_of_e1rm))
            / 4.0
    }
    fn maxreps_epley(fraction_of_e1rm: f64) -> f64 {
        (1.0 / fraction_of_e1rm - 1.0) * 30.0
    }
    fn maxreps_brzycki(fraction_of_e1rm: f64) -> f64 {
        37.0 - (fraction_of_e1rm * 36.0)
    }
    fn maxreps_adams(fraction_of_e1rm: f64) -> f64 {
        (1.0 - fraction_of_e1rm) / 0.02
    }
    fn maxreps_baechle(fraction_of_e1rm: f64) -> f64 {
        (1.0 / fraction_of_e1rm - 1.0) / 0.033
    }

    pub fn estimated_1rm(weight_lifted: f64, reps: f64, e1rm_formula: E1rmFormula) -> f64 {
        weight_lifted * e1rm_formula.e1rm(reps.clamp(0.0, 12.0))
    }

    pub fn estimated_nrm(e1rm: f64, reps: f64, e1rm_formula: E1rmFormula) -> f64 {
        e1rm / e1rm_formula.e1rm(reps.clamp(0.0, 12.0))
    }

    pub fn rir(&self, fraction_of_e1rm: f64, reps: f64) -> f64 {
        self.maxreps(fraction_of_e1rm) - reps
    }

    pub fn rpe(&self, fraction_of_e1rm: f64, reps: f64) -> f64 {
        10.0 - self.rir(fraction_of_e1rm, reps)
    }

    pub fn fraction_of_e1rm_at_reps_and_rpe(&self, reps: f64, rpe: f64) -> f64 {
        let maxreps = reps + (10.0 - rpe);

        1.0 / self.e1rm(maxreps)
    }

    pub fn estimated_set_rpe(&self, fraction_of_e1rm: f64, reps: f64) -> f64 {
        10.0 - (self.maxreps(fraction_of_e1rm) - reps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3.0, E1rmFormula::Epley, 1.1)]
    #[case(6.0, E1rmFormula::Epley, 1.2)]
    #[case(3.0, E1rmFormula::Brzycki, 1.058823)]
    #[case(6.0, E1rmFormula::Brzycki, 1.161290)]
    #[case(3.0, E1rmFormula::Adams, 1.063829)]
    #[case(6.0, E1rmFormula::Adams, 1.136666)]
    #[case(3.0, E1rmFormula::Baechle, 1.099)]
    #[case(6.0, E1rmFormula::Baechle, 1.198)]
    fn conversion(#[case] reps: f64, #[case] formula: E1rmFormula, #[case] expected: f64) {
        use approx::assert_relative_eq;

        assert_relative_eq!(formula.e1rm(reps), expected, epsilon = 0.001);
    }

    #[rstest]
    #[case(0.9, 3.0, E1rmFormula::Epley, 0.3333)]
    #[case(0.9, 3.0, E1rmFormula::Brzycki, 1.6000)]
    #[case(0.9, 3.0, E1rmFormula::Adams, 1.9999)]
    #[case(0.9, 3.0, E1rmFormula::Baechle, 0.3670)]
    #[case(0.9, 3.0, E1rmFormula::ALL, 1.0750)]
    #[case(0.8, 6.0, E1rmFormula::Epley, 1.5000)]
    #[case(0.8, 6.0, E1rmFormula::Brzycki, 2.1999)]
    #[case(0.8, 6.0, E1rmFormula::Adams, 3.9999)]
    #[case(0.8, 6.0, E1rmFormula::Baechle, 1.5757)]
    #[case(0.8, 6.0, E1rmFormula::ALL, 2.3189)]

    fn rir(
        #[case] fraction_of_e1rm: f64,
        #[case] reps: f64,
        #[case] formula: E1rmFormula,
        #[case] expected: f64,
    ) {
        use approx::assert_relative_eq;

        assert_relative_eq!(
            formula.rir(fraction_of_e1rm, reps),
            expected,
            epsilon = 0.001
        );
    }

    #[rstest]
    #[case(5.0, 9.0, E1rmFormula::ALL, 0.8518)]

    fn fraction_of_e1rm_at_reps_and_rpe(
        #[case] reps: f64,
        #[case] rpe: f64,
        #[case] formula: E1rmFormula,
        #[case] expected: f64,
    ) {
        use approx::assert_relative_eq;

        assert_relative_eq!(
            formula.fraction_of_e1rm_at_reps_and_rpe(reps, rpe),
            expected,
            epsilon = 0.001
        );
    }

    #[rstest]
    #[case(0.8518, 5.0, E1rmFormula::ALL, 8.9408)]

    fn estimated_set_rpe(
        #[case] fraction_of_e1rm: f64,
        #[case] reps: f64,
        #[case] formula: E1rmFormula,
        #[case] expected: f64,
    ) {
        use approx::assert_relative_eq;

        assert_relative_eq!(
            formula.estimated_set_rpe(fraction_of_e1rm, reps),
            expected,
            epsilon = 0.001
        );
    }
}
