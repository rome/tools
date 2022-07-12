use serde::Deserialize;

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct LinterConfiguration {
    // if `false`, it disables the feature. `true` by default
    pub enabled: bool,
}

impl Default for LinterConfiguration {
    fn default() -> Self {
        Self { enabled: true }
    }
}
