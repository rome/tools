use crate::traversal::traverse;
use crate::{CliSession, TerminationDiagnostic};
use rome_console::{markup, ConsoleExt};
use rome_diagnostics::location::FileId;
use rome_diagnostics::MAXIMUM_DISPLAYABLE_DIAGNOSTICS;
use rome_fs::RomePath;
use rome_service::workspace::{
    FeatureName, FixFileMode, FormatFileParams, Language, OpenFileParams, SupportsFeatureParams,
};
use std::path::PathBuf;

/// Useful information during the traversal of files and virtual content
pub(crate) struct Execution {
    /// How the information should be collected and reported
    report_mode: ReportMode,

    /// The modality of execution of the traversal
    traversal_mode: TraversalMode,

    /// The maximum number of diagnostics that can be printed in console
    max_diagnostics: u16,
}

pub(crate) enum TraversalMode {
    /// This mode is enabled when running the command `rome check`
    Check {
        /// The type of fixes that should be applied when analyzing a file.
        ///
        /// It's [None] if the `check` command is called without `--apply` or `--apply-suggested`
        /// arguments.
        fix_file_mode: Option<FixFileMode>,
    },
    /// This mode is enabled when running the command `rome ci`
    CI,
    /// This mode is enabled when running the command `rome format`
    Format {
        /// It ignores parse errors
        ignore_errors: bool,
        /// It writes the new content on file
        write: bool,
        /// An optional tuple.
        /// 1. The virtual path to the file
        /// 2. The content of the file
        stdin: Option<(PathBuf, String)>,
    },
}

/// Tells to the execution of the traversal how the information should be reported
#[derive(Copy, Clone, Default)]
pub(crate) enum ReportMode {
    /// Reports information straight to the console, it's the default mode
    #[default]
    Terminal,
    /// Reports information in JSON format
    Json,
}

impl Execution {
    pub(crate) fn new(mode: TraversalMode) -> Self {
        Self {
            report_mode: ReportMode::default(),
            traversal_mode: mode,
            max_diagnostics: MAXIMUM_DISPLAYABLE_DIAGNOSTICS,
        }
    }

    /// Creates an instance of [Execution] by passing [traversal mode](TraversalMode) and [report mode](ReportMode)
    pub(crate) fn with_report(traversal_mode: TraversalMode, report_mode: ReportMode) -> Self {
        Self {
            traversal_mode,
            report_mode,
            max_diagnostics: MAXIMUM_DISPLAYABLE_DIAGNOSTICS,
        }
    }

    /// Tells if the reporting is happening straight to terminal
    pub(crate) fn should_report_to_terminal(&self) -> bool {
        matches!(self.report_mode, ReportMode::Terminal)
    }

    pub(crate) fn traversal_mode(&self) -> &TraversalMode {
        &self.traversal_mode
    }

    pub(crate) fn get_max_diagnostics(&self) -> u16 {
        self.max_diagnostics
    }

    /// `true` only when running the traversal in [TraversalMode::Check] and `should_fix` is `true`
    pub(crate) fn as_fix_file_mode(&self) -> Option<&FixFileMode> {
        if let TraversalMode::Check { fix_file_mode, .. } = &self.traversal_mode {
            fix_file_mode.as_ref()
        } else {
            None
        }
    }

    pub(crate) const fn is_ci(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::CI { .. })
    }

    pub(crate) const fn is_check(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::Check { .. })
    }

    pub(crate) const fn is_format(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::Format { .. })
    }

    pub(crate) fn as_stdin_file(&self) -> Option<&(PathBuf, String)> {
        match &self.traversal_mode {
            TraversalMode::Format { stdin, .. } => stdin.as_ref(),
            _ => None,
        }
    }

    /// Returns the subcommand of the [traversal mode](TraversalMode) execution
    pub(crate) fn traversal_mode_subcommand(&self) -> &'static str {
        match self.traversal_mode {
            TraversalMode::Check { .. } => "check",
            TraversalMode::CI { .. } => "ci",
            TraversalMode::Format { .. } => "format",
        }
    }
}

/// Based on the [mode](ExecutionMode), the function might launch a traversal of the file system
/// or handles the stdin file.
pub(crate) fn execute_mode(
    mut mode: Execution,
    mut session: CliSession,
) -> Result<(), TerminationDiagnostic> {
    let max_diagnostics: Option<u16> = session
        .args
        .opt_value_from_str("--max-diagnostics")
        .map_err(|source| TerminationDiagnostic::parse_error("--max-diagnostics", source))?;

    mode.max_diagnostics = if let Some(max_diagnostics) = max_diagnostics {
        if max_diagnostics > MAXIMUM_DISPLAYABLE_DIAGNOSTICS {
            return Err(TerminationDiagnostic::overflown_argument(
                "--max-diagnostics",
                MAXIMUM_DISPLAYABLE_DIAGNOSTICS,
            ));
        }

        max_diagnostics
    } else {
        // The command `rome check` gives a default value of 20.
        // In case of other commands that pass here, we limit to 50 to avoid to delay the terminal.
        match &mode.traversal_mode {
            TraversalMode::Check { .. } => 20,
            TraversalMode::CI | TraversalMode::Format { .. } => 50,
        }
    };

    // don't do any traversal if there's some content coming from stdin
    if let Some((path, content)) = mode.as_stdin_file() {
        let workspace = &*session.app.workspace;
        let console = &mut *session.app.console;
        let rome_path = RomePath::new(path, FileId::zero());

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

                console.log(markup! {
                    {printed.as_code()}
                });
            } else {
                console.log(markup! {
                    {content}
                });
                console.error(markup!{
                    <Warn>"The content was not formatted because the formatter is currently disabled."</Warn>
                })
            }
        }

        Ok(())
    } else {
        traverse(mode, session)
    }
}
