//! A simple implementation of feature flags.
//!
//! Feature flags are created using the [declare_feature_flags] macro.
//!
//! ```ignore
//! declare_feature_flags!(
//!     /// A feature that is not finished yet
//!     unfinished_feature,
//!     /// A new unstable approach to parsing
//!     modified_parsing
//! )
//! ```
//!
//! Flags are retrieved using the [unstable] function and checked by calling the method
//! matching the name of the feature.
//!
//! ```ignore
//! if rome_flags::unstable().unfinished_feature() {
//!     // Do something
//! }
//! ```
//!
//! The current implementation doesn't allow for runtime modification of flags. They can only
//! be set once using [set_unstable_flags].

use std::str::FromStr;

use once_cell::sync::OnceCell;

static FLAGS: OnceCell<FeatureFlags> = OnceCell::new();

/// Returns the feature flags for this program run. Flags are all disabled until [set_unstable_flags] is called.
pub fn unstable() -> &'static FeatureFlags {
    FLAGS.get().unwrap_or(&FeatureFlags::NONE)
}

/// Sets feature flags for this program run if they weren't previously set.
pub fn set_unstable_flags(flags: FeatureFlags) {
    if FLAGS.set(flags).is_err() {
        eprintln!("Attempted to set rome_feature unstable flags more than once")
    }
}

macro_rules! declare_feature_flags {
    ( $( $(#[doc = $doc:tt])* $feature:ident ),* )=> {
        #[derive(Debug, Default)]
        /// State of all feature flags
        pub struct FeatureFlags {
            $(
                $feature: bool,
            )*
        }

        impl FeatureFlags {
            pub const ALL: Self = Self {
                $( $feature: true, )*
            };

            pub const NONE: Self = Self {
                $( $feature: false, )*
            };

            $(
                $(#[doc = $doc])*
                pub fn $feature(&self) -> bool { self.$feature }
            )*
        }

        impl FromStr for FeatureFlags {
            type Err = String;

        // Must be a comma-separated list (no spaces) of features that exactly match
        // the features declared with the declare_feature_flags macro
        fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut unknown_features = Vec::new();
                #[allow(unused_mut)]
                let mut flags = FeatureFlags::default();
                for feature in s.split(',') {
                    match feature {
                        $( stringify!($feature) => flags.$feature = true, )*
                        unknown => unknown_features.push(unknown)
                    }
                }
                if unknown_features.is_empty() {
                    Ok(flags)
                } else {
                    Err(format!("Unknown features: {}", unknown_features.join(",")))
                }
            }
        }
    };
}

// Flags for unstable features are declared below and are intended to be temporary.
// When it's no longer necessary to gate a feature, remove the flag from this list.
declare_feature_flags!();
