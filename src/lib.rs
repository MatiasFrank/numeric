//! Numeric Rust provides a foundation for doing scientific computing with Rust. It aims to be for
//! Rust what Numpy is for Python.
//!

pub mod traits;
pub mod tensor;
pub mod math;
pub mod random;

// Lift commonly used functions into the numeric namespace
pub use tensor::{Tensor, AxisIndex, Ellipsis, StridedSlice, Index, Full, NewAxis};

pub use math::{log, ln, log10, log2, sin, cos, tan, asin, acos, atan, exp_m1, exp, exp2,
               ln_1p, sinh, cosh, tanh, asinh, acosh, atanh, atan2, sqrt,
               floor, ceil, round, trunc, fract, abs, signum, powf, powi,
               is_nan, is_finite, is_infinite, is_normal,
               is_sign_positive, is_sign_negative};

pub use random::RandomState;
