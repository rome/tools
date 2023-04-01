//! In here, there are the operations that run via standard input
//!
use crate::execute::Execution;
use crate::{CliDiagnostic, CliSession};
use rome_console::{markup, ConsoleExt};
use rome_fs::RomePath;
use rome_service::workspace::{
    FeatureName, FormatFileParams, Language, OpenFileParams, SupportsFeatureParams,
};

pub(crate) fn run<'a>(
    session: CliSession,
    mode: &'a Execution,
    rome_path: RomePath,
    content: &'a str,
) -> Result<(), CliDiagnostic> {
    let workspace = &*session.app.workspace;
    let console = &mut *session.app.console;

    if mode.is_format() {
        let unsupported_format_reason = workspace
            .supports_feature(SupportsFeatureParams {
                path: rome_path.clone(),
                feature: FeatureName::Format,
            })?
            .reason;
        if unsupported_format_reason.is_none() {
            workspace.open_file(OpenFileParams {
                path: rome_path.clone(),
                version: 0,
                content: content.into(),
                language_hint: Language::default(),
            })?;
            let printed = workspace.format_file(FormatFileParams { path: rome_path })?;

            console.append(markup! {
                {printed.as_code()}
            });
        } else {
            console.append(markup! {
                {content}
            });
            console.error(markup!{
                    <Warn>"The content was not formatted because the formatter is currently disabled."</Warn>
                })
        }
    }
    Ok(())
}
