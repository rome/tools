use rome_service::Configuration;
use schemars::schema_for;
use serde_json::to_string_pretty;
use xtask::{project_root, Mode, Result};
use xtask_codegen::update;

pub(crate) fn generate_configuration_schema(mode: Mode) -> Result<()> {
    let schema_path_vscode = project_root().join("editors/vscode/configuration_schema.json");
    let schema_path_npm = project_root().join("npm/rome/configuration_schema.json");

    let schema = schema_for!(Configuration);
    let json_schema = to_string_pretty(&schema)?;
    update(&schema_path_vscode, &json_schema, &mode)?;
    update(&schema_path_npm, &json_schema, &mode)?;

    Ok(())
}
