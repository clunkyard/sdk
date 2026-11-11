//! The hello's formulas: f(x) = round(clamp(add + k × x^e, yMin, yMax)),
//! with x first clamped to [xMin, xMax] (reference.js evaluate).

use crate::json::Value;
use crate::maths::{floor, sqrt};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Formula {
    pub add: f64,
    pub k: f64,
    pub e: f64,
    pub x_min: Option<f64>,
    pub x_max: Option<f64>,
    pub y_min: Option<f64>,
    pub y_max: Option<f64>,
    /// "nearest" rounds half up, floor(y + 0.5); anything else does not round.
    pub nearest: bool,
}

impl Formula {
    pub fn from_json(value: &Value) -> Option<Formula> {
        let number = |key: &str| value.get(key).and_then(Value::as_f64);
        Some(Formula {
            add: number("add")?,
            k: number("k")?,
            e: number("e")?,
            x_min: number("xMin"),
            x_max: number("xMax"),
            y_min: number("yMin"),
            y_max: number("yMax"),
            nearest: value.get("round").and_then(Value::as_str) == Some("nearest"),
        })
    }

    /// None for an exponent outside the six a formula may use.
    pub fn evaluate(&self, mut x: f64) -> Option<f64> {
        if let Some(low) = self.x_min {
            if x < low {
                x = low;
            }
        }
        if let Some(high) = self.x_max {
            if x > high {
                x = high;
            }
        }
        let mut y = self.add + self.k * power(x, self.e)?;
        if let Some(low) = self.y_min {
            if y < low {
                y = low;
            }
        }
        if let Some(high) = self.y_max {
            if y > high {
                y = high;
            }
        }
        Some(if self.nearest { floor(y + 0.5) } else { y })
    }
}

/// x^e for the six exponents a formula may use.
pub fn power(x: f64, e: f64) -> Option<f64> {
    Some(if e == 1.0 {
        x
    } else if e == 2.0 {
        x * x
    } else if e == -1.0 {
        1.0 / x
    } else if e == -2.0 {
        1.0 / (x * x)
    } else if e == 0.5 {
        sqrt(x)
    } else if e == -0.5 {
        1.0 / sqrt(x)
    } else {
        return None;
    })
}
