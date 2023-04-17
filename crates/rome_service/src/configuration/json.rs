use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JsonConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_comments: Option<IndexSet<String>>,
}

impl JsonConfiguration {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &["allowComments"];
}
