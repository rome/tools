//! In here, there are the operations that run via standard input
//!
use crate::execute::Execution;
use crate::{CliDiagnostic, CliSession};
use rome_console::{markup, ConsoleExt};
use rome_fs::RomePath;

use crate::cli_options::CliOptions;
use crate::execute::diagnostics::{ContentDiffAdvice, FormatDiffDiagnostic};
use rome_diagnostics::{PrintDiagnostic, MAXIMUM_DISPLAYABLE_DIAGNOSTICS};
use rome_service::workspace::{
    ChangeFileParams, FeatureName, FeaturesBuilder, FixFileParams, FormatFileParams, Language,
    OpenFileParams, OrganizeImportsParams, PullDiagnosticsParams, RuleCategories,
    SupportsFeatureParams,
};
use std::borrow::Cow;

pub(crate) fn run<'a>(
    session: CliSession,
    mode: &'a Execution,
    rome_path: RomePath,
    content: &'a str,
    cli_options: &CliOptions,
) -> Result<(), CliDiagnostic> {
    let workspace = &*session.app.workspace;
    let console = &mut *session.app.console;
    let mut version = 0;

    if mode.is_format() {
        let file_features = workspace.file_features(SupportsFeatureParams {
            path: rome_path.clone(),
            feature: FeaturesBuilder::new().with_formatter().build(),
        })?;
        if file_features.supports_for(&FeatureName::Format) {
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
        let mut diagnostics = Vec::new();
        let mut new_content = Cow::Borrowed(content);

        workspace.open_file(OpenFileParams {
            path: rome_path.clone(),
            version: 0,
            content: content.into(),
            language_hint: Language::default(),
        })?;
        // apply fix file of the linter
        let file_features = workspace.file_features(SupportsFeatureParams {
            path: rome_path.clone(),
            feature: FeaturesBuilder::new()
                .with_linter()
                .with_organize_imports()
                .with_formatter()
                .build(),
        })?;
        if let Some(fix_file_mode) = mode.as_fix_file_mode() {
            if file_features.supports_for(&FeatureName::Lint) {
                let fix_file_result = workspace.fix_file(FixFileParams {
                    fix_file_mode: *fix_file_mode,
                    path: rome_path.clone(),
                })?;
                if fix_file_result.code != new_content {
                    version += 1;
                    workspace.change_file(ChangeFileParams {
                        content: fix_file_result.code.clone(),
                        path: rome_path.clone(),
                        version,
                    })?;
                    new_content = Cow::Owned(fix_file_result.code);
                }
            }

            if file_features.supports_for(&FeatureName::OrganizeImports) {
                let result = workspace.organize_imports(OrganizeImportsParams {
                    path: rome_path.clone(),
                })?;
                if result.code != new_content {
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

        if !mode.is_check_apply_unsafe() {
            let result = workspace.pull_diagnostics(PullDiagnosticsParams {
                categories: RuleCategories::LINT | RuleCategories::SYNTAX,
                path: rome_path.clone(),
                max_diagnostics: cli_options
                    .max_diagnostics
                    .unwrap_or(MAXIMUM_DISPLAYABLE_DIAGNOSTICS)
                    as u64,
            })?;
            diagnostics.extend(result.diagnostics);
        }

        if file_features.supports_for(&FeatureName::Format) {
            let printed = workspace.format_file(FormatFileParams {
                path: rome_path.clone(),
            })?;
            if mode.is_check_apply() || mode.is_check_apply_unsafe() {
                if printed.as_code() != new_content {
                    new_content = Cow::Owned(printed.into_code());
                }
            } else {
                let diagnostic = FormatDiffDiagnostic {
                    file_name: &rome_path.display().to_string(),
                    diff: ContentDiffAdvice {
                        new: printed.as_code(),
                        old: content,
                    },
                };
                diagnostics.push(rome_diagnostics::serde::Diagnostic::new(diagnostic));
            }
        }

        match new_content {
            Cow::Borrowed(_) => {}
            Cow::Owned(new_content) => {
                console.append(markup! {
                    {new_content}
                });
            }
        }
        if !diagnostics.is_empty() {
            for diag in diagnostics {
                console.error(markup! {
                    {PrintDiagnostic::simple(&diag)}
                })
            }
        }
    }
    Ok(())
}
