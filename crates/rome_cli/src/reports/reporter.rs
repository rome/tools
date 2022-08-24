use crate::reports::check::{LinterReport, LinterReportFileDetail, LinterReportSummary};
use crate::{
    Execution, FormatterReport, FormatterReportFileDetail, FormatterReportSummary, Report,
    TraversalMode,
};
use indexmap::IndexMap;
use rome_console::codespan::Severity;
use rome_console::fmt::{Display, Formatter};
use rome_console::{markup, Console, ConsoleExt};
use rome_diagnostics::file::{Files, SimpleFile};
use rome_diagnostics::{Diagnostic, DiagnosticHeader};
use rome_service::workspace::FeatureName;
use rome_service::{ConfigurationError, RomeError};
use serde::ser::SerializeMap;
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReportDiagnosticKind {
    ConsoleDiagnostic(Diagnostic),
    MinifiedDiagnostic(ReportDiagnostic),
    Diff(ReportDiff),
}

/// Information computed from a [diagnostic][rome_diagnostics::Diagnostic]
#[derive(Debug, Serialize)]
pub struct ReportDiagnostic {
    /// Severity of the [diagnostic][rome_diagnostics::Diagnostic]
    pub severity: Severity,
    /// The code of the [diagnostic][rome_diagnostics::Diagnostic]
    pub code: Option<String>,
    /// The title of the [diagnostic][rome_diagnostics::Diagnostic]
    pub title: String,
}

/// Information computed from a diff result
#[derive(Debug, Serialize)]
pub struct ReportDiff {
    /// The severity fo the diff
    pub severity: Severity,
    /// How was the code before the command
    pub before: String,
    /// How is the code after the command
    pub after: String,
}

/// The kind of reporter
enum ReporterKind {
    /// The reporter will emit messages that will fit the terminal
    Terminal,

    /// The reporter will emit messages that will fit the JSON format
    Json,
}

#[derive(Serialize)]
pub struct Reporter<'ctx> {
    #[serde(skip)]
    kind: ReporterKind,
    #[serde(skip)]
    execution: &'ctx Execution,
    #[serde(serialize_with = "serialize_diagnostics")]
    diagnostics: IndexMap<SimpleFile, Vec<ReportDiagnosticKind>>,
    diagnostics_not_printed: usize,
    features: ReportFeatures,
    #[serde(skip)]
    duration: Duration,
}

#[derive(Debug)]
pub enum ReportKind {
    Formatter(String, FormatterReportFileDetail),
    Linter(String, LinterReportFileDetail),
}

impl<'ctx> Reporter<'ctx> {
    pub(crate) fn new_console(execution: &'ctx Execution) -> Self {
        Self {
            kind: ReporterKind::Terminal,
            execution,
            diagnostics: IndexMap::default(),
            diagnostics_not_printed: 0,
            features: ReportFeatures::from_execution(execution),
            duration: Duration::default(),
        }
    }

    pub(crate) fn new_json(execution: &'ctx Execution) -> Self {
        Self {
            kind: ReporterKind::Json,
            execution,
            diagnostics: IndexMap::default(),
            diagnostics_not_printed: 0,
            features: ReportFeatures::from_execution(execution),
            duration: Duration::default(),
        }
    }

    pub fn add_feature(&mut self, feature: FeatureName) {
        match feature {
            FeatureName::Format => {
                self.features.0.insert(
                    feature,
                    FeatureReporter::Formatter(FormatterReport::default()),
                );
            }

            FeatureName::Lint => {
                self.features
                    .0
                    .insert(feature, FeatureReporter::Check(LinterReport::default()));
            }
        };
    }

    /// It tracks a generic diagnostic
    pub fn push_error(&mut self, file: SimpleFile, err: ReportDiagnosticKind) {
        let mut diagnostics = self.diagnostics.get_mut(&file);
        if let Some(diagnostics) = diagnostics {
            diagnostics.push(err);
        } else {
            self.diagnostics.insert(file, vec![err]);
        }
    }
    /// Creates or updates a stat
    pub fn push_detail_report(&mut self, stat: ReportKind) {
        match &stat {
            ReportKind::Formatter(_, _) => {
                self.features.push_detail(FeatureName::Format, stat);
            }
            ReportKind::Linter(_, _) => {
                self.features.push_detail(FeatureName::Lint, stat);
            }
        }
    }

    pub(crate) fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn update_formatter_summary(&mut self, summary: FormatterReportSummary) {
        self.features
            .update_formatter_summary(FeatureName::Format, summary)
    }

    pub fn update_linter_summary(&mut self, summary: LinterReportSummary) {
        self.features
            .update_linter_summary(FeatureName::Lint, summary)
    }

    pub fn report_summary(&mut self, console: &mut dyn Console) -> Result<(), RomeError> {
        match self.kind {
            ReporterKind::Terminal => {
                self.features
                    .report_summary_to_console(&self.execution, console, &self.duration);

                if !self.execution.is_ci() && self.diagnostics_not_printed > 0 {
                    console.log(markup! {
                    <Warn>"The number of diagnostics exceeds the number allowed by Rome.\n"</Warn>
                    <Info>"Diagnostics not shown: "</Info><Emphasis>{self.diagnostics_not_printed}</Emphasis><Info>"."</Info>
                })
                }
            }
            ReporterKind::Json => {
                let result = serde_json::to_string(&self).map_err(|_| {
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
                    ReportDiagnosticKind::ConsoleDiagnostic(diag) => {
                        console.error(markup! {
                            {diag.display(file)}
                        });
                    }
                    ReportDiagnosticKind::Diff(header) => {}
                    _ => {}
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

pub(crate) fn serialize_diagnostics<S>(
    diagnostics: &IndexMap<SimpleFile, Vec<ReportDiagnosticKind>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    let mut map = s.serialize_map(Some(diagnostics.len()))?;
    let iter = diagnostics.into_iter();
    for (file_name, diagnostics) in iter {
        let name = file_name.name(0).expect("File not found");
        map.serialize_entry(name, diagnostics)?;
    }

    map.end()
}
#[derive(Serialize)]
pub struct ReportFeatures(IndexMap<FeatureName, FeatureReporter>);

impl ReportFeatures {
    /// It prepares the reports based on the type of the [execution](Execution)
    fn from_execution(execution: &Execution) -> Self {
        let mut map = IndexMap::default();
        match execution.traversal_mode() {
            TraversalMode::Check { .. } => {
                map.insert(
                    FeatureName::Lint,
                    FeatureReporter::Check(LinterReport::default()),
                );
            }
            TraversalMode::CI => {
                map.insert(
                    FeatureName::Lint,
                    FeatureReporter::Check(LinterReport::default()),
                );
                map.insert(
                    FeatureName::Format,
                    FeatureReporter::Formatter(FormatterReport::default()),
                );
            }
            TraversalMode::Format { .. } => {
                map.insert(
                    FeatureName::Format,
                    FeatureReporter::Formatter(FormatterReport::default()),
                );
            }
        };

        Self(map)
    }

    fn count(&self, feature: FeatureName) -> &usize {
        self.0.get(&feature).expect("Feature not supported").count()
    }

    pub fn push_detail(&mut self, feature: FeatureName, kind: ReportKind) {
        let mut reporter = self.0.get_mut(&feature);
        match kind {
            ReportKind::Formatter(path, details) => {
                if let Some(FeatureReporter::Formatter(reporter)) = &mut reporter {
                    reporter.insert_file_content(path, details);
                }
            }
            ReportKind::Linter(path, details) => {
                if let Some(FeatureReporter::Check(reporter)) = &mut reporter {
                    reporter.insert_file_content(path, details);
                }
            }
        }
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
        let mut reporter = self.0.get_mut(&feature);

        if let Some(FeatureReporter::Formatter(reporter)) = &mut reporter {
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

        let mut reporter = self.0.get_mut(&feature);

        if let Some(FeatureReporter::Check(reporter)) = &mut reporter {
            reporter.set_summary(summary)
        }
    }

    pub(crate) fn report_summary_to_console(
        &self,
        execution: &Execution,
        console: &mut dyn Console,
        duration: &Duration,
    ) {
        let iter = self.0.iter();
        for (_, report) in iter {
            match report {
                FeatureReporter::Formatter(report) => {
                    report.report_to_console(execution, console, duration);
                }
                FeatureReporter::Check(report) => {
                    report.report_to_console(execution, console, duration);
                }
            }
        }

        // if let Some(feature_report) = self.0.get(&feature) {
        //     feature_report.report_summary_to_console(duration, execution, console);
        // }
    }
}

// impl Default for ReportFeatures {
//     fn default() -> Self {
//         let mut features = IndexMap::default();
//
//         features.insert(
//             FeatureName::Format,
//             FeatureReporter::Formatter(FormatterReport::default()),
//         );
//         features.insert(
//             FeatureName::Lint,
//             FeatureReporter::Check(CheckReport::default()),
//         );
//
//         Self(features)
//     }
// }

#[derive(Debug, Serialize)]
pub enum FeatureReporter {
    Formatter(FormatterReport),
    Check(LinterReport),
}

impl FeatureReporter {
    fn count(&self) -> &usize {
        match self {
            FeatureReporter::Formatter(f_report) => f_report.formatted(),
            FeatureReporter::Check(c_report) => c_report.count(),
        }
    }

    fn report_summary_to_console(
        &self,
        duration: &Duration,
        execution: &Execution,
        console: &mut dyn Console,
    ) {
        match self {
            FeatureReporter::Formatter(f_report) => {
                f_report.report_to_console(execution, console, duration);
            }
            FeatureReporter::Check(c_report) => {
                c_report.report_to_console(execution, console, duration)
            }
        }
    }

    fn report_to_json(
        &self,
        duration: &Duration,
        execution: &Execution,
        console: &mut dyn Console,
    ) -> Result<(), RomeError> {
        match self {
            FeatureReporter::Formatter(f_report) => {
                let report = f_report.report_to_json()?;
                Ok(())
            }
            FeatureReporter::Check(c_report) => {
                c_report.report_to_json(console);
                Ok(())
            }
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ReporterSummary {
    pub(crate) skipped: usize,
    pub(crate) duration: Option<Duration>,
    pub(crate) count: usize,
}

mod test {
    use crate::reports::check::{LinterReportFileDetail, LinterReportSummary};
    use crate::reports::reporter::Reporter;
    use crate::{FormatterReportFileDetail, FormatterReportSummary, TraversalMode};
    use rome_console::fmt::{Formatter, Termcolor};
    use rome_console::{BufferConsole, Markup};
    use rome_diagnostics::termcolor::NoColor;

    pub fn markup_to_string(markup: Markup) -> String {
        let mut buffer = Vec::new();
        let mut write = Termcolor(NoColor::new(&mut buffer));
        let mut fmt = Formatter::new(&mut write);
        fmt.write_markup(markup).unwrap();

        String::from_utf8(buffer).unwrap()
    }

    #[test]
    fn formatter_ok_terminal() {
        let mut console = BufferConsole::default();
        let execution = Execution::new(TraversalMode::Format {
            ignore_errors: false,
            write: false,
            stdin: None,
        });
        let mut reporter = Reporter::new_console(&execution);

        reporter.push_detail_report(ReportKind::Formatter(
            "index.js".to_string(),
            FormatterReportFileDetail {
                formatted_content: Some("statement();".to_string()),
            },
        ));

        reporter.push_detail_report(ReportKind::Formatter(
            "test.js".to_string(),
            FormatterReportFileDetail {
                formatted_content: Some("let a;".to_string()),
            },
        ));

        reporter.update_formatter_summary(FormatterReportSummary { formatted: 2 });

        let result = reporter.report_summary(&mut console);

        assert!(result.is_ok());

        let messages = console.out_buffer;
        eprintln!("{:?}", messages);

        let content = markup_to_string(markup! {
            {messages[0].content}
        });
        assert!(content.contains("Compared 2 files"));
    }

    #[test]
    fn linter_ok_terminal() {
        let mut console = BufferConsole::default();
        let execution = Execution::new(TraversalMode::Check {
            max_diagnostics: 10,
            fix_file_mode: None,
        });
        let mut reporter = Reporter::new_console(&execution);

        reporter.push_detail_report(ReportKind::Linter(
            "index.js".to_string(),
            LinterReportFileDetail {
                rule_name: "js/noUnusedVariables".to_string(),
                safe_fix: None,
                suggested_fix: None,
            },
        ));
        reporter.push_detail_report(ReportKind::Linter(
            "index.js".to_string(),
            LinterReportFileDetail {
                rule_name: "js/noDeadCode".to_string(),
                safe_fix: None,
                suggested_fix: None,
            },
        ));

        reporter.update_linter_summary(LinterReportSummary {
            skipped: 0,
            count: 2,
        });

        let result = reporter.report_summary(&mut console);

        assert!(result.is_ok());

        let messages = console.out_buffer;
        eprintln!("{:?}", messages);

        let content = markup_to_string(markup! {
            {messages[0].content}
        });
        assert!(content.contains("Checked 2 files"));
    }
}
