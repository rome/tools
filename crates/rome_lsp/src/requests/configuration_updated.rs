use serde::{Deserialize, Serialize};

pub const CHANGE_CONFIG_REQUEST: &str = "rome_lsp/configurationUpdated";

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationUpdatedParams {}
