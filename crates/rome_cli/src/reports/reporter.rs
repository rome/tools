use crate::reports::linter::{LinterReport, LinterReportSummary};
use crate::{Execution, FormatterReport, FormatterReportSummary, TraversalMode};
use indexmap::IndexMap;
use rome_console::fmt::{Display, Formatter};
use rome_console::{markup, Console, ConsoleExt};
use rome_diagnostics::file::SimpleFile;
use rome_diagnostics::{Diagnostic, DiagnosticHeader};
use rome_service::workspace::FeatureName;
use rome_service::{ConfigurationError, RomeError};
use serde::Serialize;
use std::time::Duration;

pub enum ReportDiagnosticKind {
    Diagnostic(Diagnostic),
    Header(DiagnosticHeader<'static>),
}

enum ReporterKind {
    Console,

    Json,
}

pub struct Reporter<'ctx> {
    kind: ReporterKind,
    execution: &'ctx Execution,
    diagnostics: IndexMap<SimpleFile, Vec<ReportDiagnosticKind>>,
    diagnostics_not_printed: usize,
    features: ReportFeatures,
}

pub struct ReportFeatures(IndexMap<FeatureName, FeatureReporter>);

impl ReportFeatures {
    fn count(&self, feature: FeatureName) -> usize {
        self.0.get(&feature).expect("Feature not supported").count()
    }

    pub fn update_formatter_summary(
        &mut self,
        feature: FeatureName,
        summary: FormatterReportSummary,
    ) {
        debug_assert_eq!(
            feature,
            FeatureName::Format,
            "The feature you're passed is not the formatter, it's {:?}",
            feature
        );
        let reporter = self.0.get(&feature);

        if let Some(FeatureReporter::Formatter(mut reporter)) = reporter {
            reporter.set_summary(summary)
        }
    }

    pub fn update_linter_summary(&mut self, feature: FeatureName, summary: LinterReportSummary) {
        debug_assert_eq!(
            feature,
            FeatureName::Lint,
            "The feature you're passed is not the linter, it's {:?}",
            feature
        );

        let reporter = self.0.get(&feature);

        if let Some(FeatureReporter::Linter(mut reporter)) = reporter.as_mut() {
            reporter.set_summary(summary)
        }
    }

    pub fn print_summary(
        &self,
        feature: FeatureName,
        execution: &Execution,
        console: &mut dyn Console,
    ) {
        // match feature {
        //     FeatureName::Format => {
        //         self.
        //     }
        //     FeatureName::Lint => {}
        // }

        if let Some(feature_report) = self.0.get(&feature) {}
    }
}

impl Default for ReportFeatures {
    fn default() -> Self {
        let mut features = IndexMap::default();

        features.insert(
            FeatureName::Format,
            FeatureReporter::Formatter(FormatterReport::default()),
        );
        features.insert(
            FeatureName::Lint,
            FeatureReporter::Linter(LinterReport::default()),
        );

        Self(features)
    }
}

#[derive(Debug)]
pub enum FeatureReporter {
    Formatter(FormatterReport),
    Linter(LinterReport),
}

impl FeatureReporter {
    fn count(&self) -> usize {
        match self {
            FeatureReporter::Formatter(r) => r.summary.count,
            FeatureReporter::Linter(_) => unimplemented!("linter doesn't have count"),
        }
    }

    fn print_summary(&self, duration: &Duration, execution: &Execution, console: &mut dyn Console) {
        match self {
            FeatureReporter::Formatter(f_reporter) => {
                f_reporter
                    .summary
                    .report_to_console(execution, console, duration);
            }
            FeatureReporter::Linter(_) => {}
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ReporterSummary {
    pub(crate) skipped: usize,
    pub(crate) duration: Option<Duration>,
    pub(crate) count: usize,
}

impl<'ctx> Reporter<'ctx> {
    pub fn new_console(execution: &'ctx Execution) -> Self {
        Self {
            kind: ReporterKind::Console,
            execution,
            diagnostics: IndexMap::default(),
            diagnostics_not_printed: 0,
            features: ReportFeatures::default(),
        }
    }

    pub fn new_json(execution: &'ctx Execution) -> Self {
        Self {
            kind: ReporterKind::Json,
            execution,
            diagnostics: IndexMap::default(),
            diagnostics_not_printed: 0,
            features: ReportFeatures::default(),
        }
    }

    fn count(&self, feature: FeatureName) -> usize {
        self.features.count(feature)
    }

    pub fn update_formatter_summary(&mut self, summary: FormatterReportSummary) {
        self.features
            .update_formatter_summary(FeatureName::Format, summary)
    }

    pub fn update_linter_summary(&mut self, summary: LinterReportSummary) {
        self.features
            .update_linter_summary(FeatureName::Lint, summary)
    }

    pub fn track_report_error(
        &mut self,
        file: SimpleFile,
        report_diagnostics: Vec<ReportDiagnosticKind>,
    ) {
        self.diagnostics.insert(file, report_diagnostics);
    }

    pub fn report_summary(&mut self, console: &mut dyn Console) -> Result<(), RomeError> {
        match self.kind {
            ReporterKind::Console => {
                match self.summary.duration {
                    Some(duration) => match &self.execution.traversal_mode() {
                        TraversalMode::Check { .. } => {
                            if self.execution.as_fix_file_mode().is_some() {
                                console.log(markup! {
                                    <Info>"Fixed "{self.count()}" files in "{duration}</Info>
                                });
                            } else {
                                console.log(markup! {
                                    <Info>"Checked "{self.count()}" files in "{duration}</Info>
                                });
                            }
                        }
                        TraversalMode::CI { .. } => {
                            console.log(markup! {
                                <Info>"Checked "{self.count()}" files in "{duration}</Info>
                            });
                        }
                        TraversalMode::Format { write: false, .. } => {}
                    },
                    _ => {}
                }
                if self.execution.is_check() && self.summary.skipped > 0 {
                    console.log(markup! {
                    <Warn>"Skipped "{self.summary.skipped}" suggested fixes.\n"</Warn>
                    <Info>"If you wish to apply the suggested fixes, use the command "<Emphasis>"rome check --apply-suggested\n"</Emphasis></Info>
                })
                }

                if !self.execution.is_ci() && self.diagnostics_not_printed > 0 {
                    console.log(markup! {
                    <Warn>"The number of diagnostics exceeds the number allowed by Rome.\n"</Warn>
                    <Info>"Diagnostics not shown: "</Info><Emphasis>{self.diagnostics_not_printed}</Emphasis><Info>"."</Info>
                })
                }
            }
            ReporterKind::Json => {
                let summary = &self.summary;
                let result = serde_json::to_string(&self.summary).map_err(|_| {
                    RomeError::Configuration(ConfigurationError::SerializationError)
                })?;

                console.log(markup! { {result} });
            }
        }

        Ok(())
    }

    pub fn report_diagnostics(&mut self, console: &mut dyn Console) -> Result<(), RomeError> {
        for (file, diagnostics) in &self.diagnostics {
            for diag in diagnostics {
                match diag {
                    ReportDiagnosticKind::Diagnostic(diag) => {
                        console.error(markup! {
                            {diag.display(file)}
                        });
                    }
                    ReportDiagnosticKind::Header(header) => console.error(markup! {
                        {header}
                    }),
                }
            }
        }
        Ok(())
    }

    pub fn report_all(&mut self, console: &mut dyn Console) -> Result<(), RomeError> {
        self.report_diagnostics(console)?;
        self.report_summary(console)
    }
}

mod test {
    use crate::reports::reporter::{
        ConsoleReporter, ConsoleReporterSummary, JsonReporter, JsonReporterSummary, Reporter,
    };
    use crate::Execution;
    use crate::TraversalMode;
    use indexmap::IndexMap;
    use rome_console::BufferConsole;

    #[test]
    fn ok() {
        let mut console = BufferConsole::default();

        let mut console_reporter = ConsoleReporter {
            execution: &Execution::new(TraversalMode::Format {
                ignore_errors: false,
                write: false,
                stdin: None,
            }),
            summary: None,
            diagnostics: IndexMap::default(),
        };

        console_reporter.update_summary(ConsoleReporterSummary {
            written_files: 1,
            skipped: 0,
        });

        let summary = console_reporter.get_summary();

        if let Some(summary) = summary {
            assert_eq!(
                summary,
                &ConsoleReporterSummary {
                    written_files: 1,
                    skipped: 0,
                }
            );
        } else {
            panic!("summary should be set");
        }
    }

    #[test]
    fn logs_ok() {
        let mut console = BufferConsole::default();

        let mut console_reporter = ConsoleReporter {
            execution: &Execution::new(TraversalMode::Format {
                ignore_errors: false,
                write: false,
                stdin: None,
            }),
            summary: None,
            diagnostics: IndexMap::default(),
        };

        console_reporter.update_summary(ConsoleReporterSummary {
            written_files: 1,
            skipped: 0,
        });

        console_reporter.report_summary(&mut console).unwrap();

        let messages = console.out_buffer;
        eprintln!("{:?}", &messages);
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn logs_ok_json() {
        let mut console = BufferConsole::default();

        let mut json_reporter = JsonReporter {
            execution: &Execution::new(TraversalMode::Format {
                ignore_errors: false,
                write: false,
                stdin: None,
            }),
            summary: Some(JsonReporterSummary {
                written_files: 1,
                skipped: 0,
            }),
            details: IndexMap::default(),
            diagnostics: IndexMap::default(),
        };

        json_reporter.update_summary(JsonReporterSummary {
            written_files: 1,
            skipped: 0,
        });

        json_reporter.report_summary(&mut console).unwrap();

        let messages = console.out_buffer;

        eprintln!("{:?}", &messages);
        assert_eq!(messages.len(), 1);
    }
}
