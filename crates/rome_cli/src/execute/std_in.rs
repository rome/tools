//! In here, there are the operations that run via standard input
//!
use crate::execute::Execution;
use crate::{CliDiagnostic, CliSession};
use rome_console::{markup, ConsoleExt};
use rome_fs::RomePath;
use rome_service::workspace::{
    ChangeFileParams, FeatureName, FixFileParams, FormatFileParams, Language, OpenFileParams,
    OrganizeImportsParams, SupportsFeatureParams,
};
use std::borrow::Cow;

pub(crate) fn run<'a>(
    session: CliSession,
    mode: &'a Execution,
    rome_path: RomePath,
    content: &'a str,
) -> Result<(), CliDiagnostic> {
    let workspace = &*session.app.workspace;
    let console = &mut *session.app.console;
    let mut version = 0;

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
    } else if mode.is_check() {
        let mut new_content = Cow::Borrowed(content);

        workspace.open_file(OpenFileParams {
            path: rome_path.clone(),
            version: 0,
            content: content.into(),
            language_hint: Language::default(),
        })?;

        if let Some(fix_file_mode) = mode.as_fix_file_mode() {
            // apply fix file of the linter
            let unsupported_lint_reason = workspace
                .supports_feature(SupportsFeatureParams {
                    path: rome_path.clone(),
                    feature: FeatureName::Lint,
                })?
                .reason;

            if unsupported_lint_reason.is_none() {
                let fix_file_result = workspace.fix_file(FixFileParams {
                    fix_file_mode: *fix_file_mode,
                    path: rome_path.clone(),
                })?;
                if &fix_file_result.code != &new_content {
                    version += 1;
                    workspace.change_file(ChangeFileParams {
                        content: fix_file_result.code.clone(),
                        path: rome_path.clone(),
                        version,
                    })?;
                    new_content = Cow::Owned(fix_file_result.code);
                }
            }

            // apply organize imports
            let unsupported_organize_imports_reason = workspace
                .supports_feature(SupportsFeatureParams {
                    path: rome_path.clone(),
                    feature: FeatureName::OrganizeImports,
                })?
                .reason;

            if unsupported_organize_imports_reason.is_none() {
                let result = workspace.organize_imports(OrganizeImportsParams {
                    path: rome_path.clone(),
                })?;
                if &result.code != &new_content {
                    version += 1;
                    workspace.change_file(ChangeFileParams {
                        content: result.code.clone(),
                        path: rome_path.clone(),
                        version,
                    })?;
                    new_content = Cow::Owned(result.code);
                }
            }
        }

        let unsupported_format_reason = workspace
            .supports_feature(SupportsFeatureParams {
                path: rome_path.clone(),
                feature: FeatureName::Format,
            })?
            .reason;

        if unsupported_format_reason.is_none() {
            let printed = workspace.format_file(FormatFileParams { path: rome_path })?;
            if printed.as_code() != &new_content {
                new_content = Cow::Owned(printed.into_code());
            }
        }

        match new_content {
            Cow::Borrowed(original) => {
                console.append(markup! {
                    {original}
                });
            }
            Cow::Owned(new_content) => {
                console.append(markup! {
                    {new_content}
                });
            }
        }
    }
    Ok(())
}
