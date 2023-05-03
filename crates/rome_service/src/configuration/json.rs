use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

use super::StringSet;
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JsonConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub allow_comments: Option<StringSet>,
}

impl JsonConfiguration {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &["allowComments"];
}
