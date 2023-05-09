mod diagnostics;
mod migrate;
mod process_file;
mod std_in;
mod traverse;

use crate::cli_options::CliOptions;
use crate::execute::traverse::traverse;
use crate::{CliDiagnostic, CliSession};
use rome_diagnostics::MAXIMUM_DISPLAYABLE_DIAGNOSTICS;
use rome_fs::RomePath;
use rome_service::workspace::{FeatureName, FixFileMode};
use std::ffi::OsString;
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

impl Execution {
    pub(crate) fn as_feature_name(&self) -> FeatureName {
        match self.traversal_mode {
            TraversalMode::Format { .. } => FeatureName::Format,
            _ => FeatureName::Lint,
        }
    }
}

#[derive(Debug)]
pub(crate) enum TraversalMode {
    /// This mode is enabled when running the command `rome check`
    Check {
        /// The type of fixes that should be applied when analyzing a file.
        ///
        /// It's [None] if the `check` command is called without `--apply` or `--apply-suggested`
        /// arguments.
        fix_file_mode: Option<FixFileMode>,
        /// An optional tuple.
        /// 1. The virtual path to the file
        /// 2. The content of the file
        stdin: Option<(PathBuf, String)>,
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
    /// This mode is enabled when running the command `rome migrate`
    Migrate {
        write: bool,
        configuration_path: PathBuf,
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

    pub(crate) const fn is_check_apply(&self) -> bool {
        matches!(
            self.traversal_mode,
            TraversalMode::Check {
                fix_file_mode: Some(FixFileMode::SafeFixes),
                ..
            }
        )
    }

    pub(crate) const fn is_check_apply_unsafe(&self) -> bool {
        matches!(
            self.traversal_mode,
            TraversalMode::Check {
                fix_file_mode: Some(FixFileMode::SafeAndUnsafeFixes),
                ..
            }
        )
    }

    pub(crate) const fn is_format(&self) -> bool {
        matches!(self.traversal_mode, TraversalMode::Format { .. })
    }

    /// Whether the traversal mode requires write access to files
    pub(crate) const fn requires_write_access(&self) -> bool {
        match self.traversal_mode {
            TraversalMode::Check { fix_file_mode, .. } => fix_file_mode.is_some(),
            TraversalMode::CI => false,
            TraversalMode::Format { write, .. } => write,
            TraversalMode::Migrate { write: dry_run, .. } => dry_run,
        }
    }

    pub(crate) fn as_stdin_file(&self) -> Option<&(PathBuf, String)> {
        match &self.traversal_mode {
            TraversalMode::Format { stdin, .. } => stdin.as_ref(),
            TraversalMode::Check { stdin, .. } => stdin.as_ref(),
            _ => None,
        }
    }

    /// Returns the subcommand of the [traversal mode](TraversalMode) execution
    pub(crate) fn traversal_mode_subcommand(&self) -> &'static str {
        match self.traversal_mode {
            TraversalMode::Check { .. } => "check",
            TraversalMode::CI { .. } => "ci",
            TraversalMode::Format { .. } => "format",
            TraversalMode::Migrate { .. } => "migrate",
        }
    }
}

/// Based on the [mode](ExecutionMode), the function might launch a traversal of the file system
/// or handles the stdin file.
pub(crate) fn execute_mode(
    mut mode: Execution,
    session: CliSession,
    cli_options: &CliOptions,
    paths: Vec<OsString>,
) -> Result<(), CliDiagnostic> {
    mode.max_diagnostics = if let Some(max_diagnostics) = cli_options.max_diagnostics {
        if max_diagnostics > MAXIMUM_DISPLAYABLE_DIAGNOSTICS {
            return Err(CliDiagnostic::overflown_argument(
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
            TraversalMode::CI | TraversalMode::Format { .. } | TraversalMode::Migrate { .. } => 50,
        }
    };

    // don't do any traversal if there's some content coming from stdin
    if let Some((path, content)) = mode.as_stdin_file() {
        let rome_path = RomePath::new(path);
        std_in::run(session, &mode, rome_path, content.as_str(), cli_options)
    } else if let TraversalMode::Migrate {
        write,
        configuration_path,
    } = mode.traversal_mode
    {
        migrate::run(session, write, configuration_path, cli_options.verbose)
    } else {
        traverse(mode, session, cli_options, paths)
    }
}
