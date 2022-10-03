use rome_service::Configuration;
use schemars::schema_for;
use serde_json::to_string_pretty;
use xtask::{project_root, Mode, Result};
use xtask_codegen::update;

pub(crate) fn generate_configuration_schema(mode: Mode) -> Result<()> {
    let schema_path = project_root().join("editors/vscode/configuration_schema.json");

    let schema = schema_for!(Configuration);
    let json_schema = to_string_pretty(&schema)?;
    update(&schema_path, &json_schema, &mode)?;

    Ok(())
}
