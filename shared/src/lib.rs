pub mod cool_ratio;
pub mod helpers;
mod settings;

pub use cool_ratio::{CoolRatio, CoolRatioT};
pub use helpers::r_to_f64;
pub use settings::{default_settings, get_test_settings, Settings};
