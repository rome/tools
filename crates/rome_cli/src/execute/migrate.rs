use crate::execute::diagnostics::{ContentDiffAdvice, MigrateDiffDiagnostic};
use crate::{CliDiagnostic, CliSession};
use rome_console::{markup, ConsoleExt};
use rome_diagnostics::PrintDiagnostic;
use rome_fs::OpenOptions;
use rome_json_parser::JsonParserConfig;
use rome_json_syntax::JsonRoot;
use rome_migrate::{migrate_configuration, ControlFlow};
use rome_rowan::AstNode;
use rome_service::workspace::FixAction;
use std::borrow::Cow;
use std::path::PathBuf;

pub(crate) fn run(
    session: CliSession,
    write: bool,
    configuration_path: PathBuf,
    verbose: bool,
) -> Result<(), CliDiagnostic> {
    let fs = &*session.app.fs;
    let open_options = if write {
        OpenOptions::default().write(true)
    } else {
        OpenOptions::default().read(true)
    };
    let mut configuration_file =
        fs.open_with_options(configuration_path.as_path(), open_options)?;
    let mut configuration_content = String::new();
    configuration_file.read_to_string(&mut configuration_content)?;
    let parsed = rome_json_parser::parse_json(&configuration_content, JsonParserConfig::default());
    let mut errors = 0;
    let mut tree = parsed.tree();
    let mut actions = Vec::new();
    loop {
        let (action, _) = migrate_configuration(
            &tree.value().unwrap(),
            configuration_path.as_path(),
            |signal| {
                let current_diagnostic = signal.diagnostic();

                if current_diagnostic.is_some() {
                    errors += 1;
                }

                if let Some(action) = signal.actions().next() {
                    return ControlFlow::Break(action);
                }

                ControlFlow::Continue(())
            },
        );

        match action {
            Some(action) => {
                if let Some((range, _)) = action.mutation.as_text_edits() {
                    tree = match JsonRoot::cast(action.mutation.commit()) {
                        Some(tree) => tree,
                        None => return Err(CliDiagnostic::check_error()),
                    };
                    actions.push(FixAction {
                        rule_name: action
                            .rule_name
                            .map(|(group, rule)| (Cow::Borrowed(group), Cow::Borrowed(rule))),
                        range,
                    });
                }
            }
            None => {
                break;
            }
        }
    }
    let console = &mut *session.app.console;
    let new_configuration_content = tree.to_string();

    if configuration_content != new_configuration_content {
        if write {
            configuration_file.set_content(tree.to_string().as_bytes())?;
            console.log(markup!{
					<Info>"The configuration "<Emphasis>{{configuration_path.display().to_string()}}</Emphasis>" has been successfully migrated"</Info>
				})
        } else {
            let file_name = configuration_path.display().to_string();
            let diagnostic = MigrateDiffDiagnostic {
                file_name: &file_name,
                diff: ContentDiffAdvice {
                    old: configuration_content.as_str(),
                    new: new_configuration_content.as_str(),
                },
            };
            console.error(markup! {
					{if verbose { PrintDiagnostic::verbose(&diagnostic) } else { PrintDiagnostic::simple(&diagnostic) }}
				});
        }
    } else {
        console.log(markup! {
            <Info>
            "Your configuration file is up to date."
            </Info>
        })
    }
    Ok(())
}
