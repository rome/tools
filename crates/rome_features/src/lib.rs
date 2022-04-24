use std::str::FromStr;

use once_cell::sync::OnceCell;

static FLAGS: OnceCell<FeatureFlags> = OnceCell::new();

/// Returns the feature flags for this program run. Flags are all disabled until [set_flags] is called.
pub fn flags() -> &'static FeatureFlags {
    FLAGS.get().unwrap_or(&FeatureFlags::NONE)
}

/// Sets feature flags for this program run if they weren't previously set.
pub fn set_flags(flags: FeatureFlags) {
    if FLAGS.set(flags).is_err() {
        eprintln!("Attempted to set rome_feature FLAGS more than once")
    }
}

macro_rules! declare_feature_flags {
    ( $( $feature:ident),* ) => {
        #[derive(Debug, Default)]
        /// State of all feature flags
        pub struct FeatureFlags {
            $( pub $feature: bool, )*
        }

        impl FeatureFlags {
            pub const ALL: Self = Self {
                $( $feature: true, )*
            };

            pub const NONE: Self = Self {
                $( $feature: false, )*
            };
        }

        impl FromStr for FeatureFlags {
            type Err = String;

        // Must be a comma-separated list (no spaces) of features that exactly match
        // the features declared with the declare_feature_flags macro
        fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut unknown_features = Vec::new();
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
// EXAMPLE: Remove dummy feature flags before merging
declare_feature_flags!(new_linebreaking, new_spacing);
