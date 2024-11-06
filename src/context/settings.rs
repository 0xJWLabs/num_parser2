//!
//! Contains contexts settings.
//!

use std::default::Default;
use std::f64::consts;

use crate::{EvalResult, Value};

/// The number of decimal places shown.
///
/// ## Examples
/// ```
/// use num_parser2::*;
///
/// let my_context = Context::new(
///     settings::Rounding::Round(4),
///     settings::AngleUnit::default(),
///     settings::DepthLimit::default()
/// );
///
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rounding {
    /// Round number to a specific decimal place.
    Round(u8),
    /// Disable rounding.
    NoRounding,
}

impl Default for Rounding {
    fn default() -> Self {
        Rounding::Round(8)
    }
}

impl Rounding {}

/// The angle unit to use.
///
/// ## Examples
/// ```
/// use num_parser2::*;
///
/// let my_context = Context::new(
///     settings::Rounding::default(),
///     settings::AngleUnit::Degree, // Or Radian or Turn
///     settings::DepthLimit::default()
/// );
///
/// ```
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AngleUnit {
    #[default]
    /// Measure angles in radians. A full turn is 2π.
    Radian,
    /// Measure angles in degrees. A full turn is 360°.
    Degree,
    /// Measure angles in turns. A full turn is 1.
    Turn,
}

impl AngleUnit {
    /// Returns the angle unit default value.
    /// Converts a value from an angle unit to another.
    pub fn convert_value(self, to: Self, value: Value) -> EvalResult<Value> {
        let as_radians = match self {
            Self::Radian => value,
            Self::Degree => {
                let division_result = value / Value::from(180);
                let division_value = match division_result {
                    Ok(val) => val,
                    Err(e) => return Err(e), // Return the error if division fails
                };
                let final_result = division_value * Value::from(consts::PI);
                let result_value = match final_result {
                    Ok(val) => val,
                    Err(e) => return Err(e), // Return the error if multiplication fails
                };
                result_value
            }
            Self::Turn => {
                let division_result = value / Value::from(0.5);
                let division_value = match division_result {
                    Ok(val) => val,
                    Err(e) => return Err(e), // Return the error if division fails
                };

                let final_result = division_value / Value::from(consts::PI);
                let result_value = match final_result {
                    Ok(val) => val,
                    Err(e) => return Err(e), // Return the error if multiplication fails
                };

                result_value
            }
        };

        Ok(match to {
            Self::Radian => as_radians,
            Self::Degree => {
                // Perform division and multiplication for Degree
                let division_result = as_radians / Value::from(consts::PI);
                let division_value = match division_result {
                    Ok(val) => val,
                    Err(e) => return Err(e), // Return error if division fails
                };

                let final_result = division_value * Value::from(180);
                match final_result {
                    Ok(val) => val,
                    Err(e) => return Err(e), // Return error if multiplication fails
                }
            }
            Self::Turn => {
                // Perform division and multiplication for Turn
                let division_result = as_radians / Value::from(consts::PI);
                let division_value = match division_result {
                    Ok(val) => val,
                    Err(e) => return Err(e), // Return error if division fails
                };

                let final_result = division_value * Value::from(0.5);
                match final_result {
                    Ok(val) => val,
                    Err(e) => return Err(e), // Return error if multiplication fails
                }
            }
        })
    }
}

/// The depth limit.
///
/// ## Examples
/// ```
/// use num_parser2::*;
///
/// let my_context = Context::new(
///     settings::Rounding::default(),
///     settings::AngleUnit::default(),
///     settings::DepthLimit::Limit(10)
/// );
///
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DepthLimit {
    /// An iteration limit.
    Limit(u32),
    /// No limit.
    ///
    /// **WARNING**: disabling limit prevents recursion control and may cause
    /// the stack to overflow causing the program to panic.
    NoLimit,
}

impl Default for DepthLimit {
    fn default() -> Self {
        DepthLimit::Limit(49)
    }
}
