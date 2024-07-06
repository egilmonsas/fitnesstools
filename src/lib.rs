//! Fitnesstools is a library of math utitilites in fitness such as
//!
//! - **Powerlifting ratings** — Common rating normalizing formulas such as Wilks, DOTS(Not implemented)
//!
//! - **1rm estimators** — Not implemented
//!
//! - **Set difficulty rating / Equivalent RPE** — Not implemented

pub enum Gender {
    Male,
    Female,
}

pub mod bw_coefficients;
pub mod estimates;

#[doc(hidden)]
mod util;
