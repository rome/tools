use crate::language::Analyze;
use crate::test_case::TestCase;
use crate::BenchmarkSummary;
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AnalyzerMeasurement {
    id: String,
    analysis: Duration,
}
pub fn benchmark_analyze_lib(case: &TestCase, analyze: &Analyze) -> BenchmarkSummary {
    let analyzer_timer = timing::start();
    analyze.analyze();
    let analyzer_duration = analyzer_timer.stop();

    BenchmarkSummary::Analyzer(AnalyzerMeasurement {
        id: case.filename().to_string(),
        analysis: analyzer_duration,
    })
}

impl AnalyzerMeasurement {
    fn total(&self) -> Duration {
        self.analysis
    }

    pub(crate) fn summary(&self) -> String {
        format!("{}, Analysis: {:?}", self.id, self.total())
    }
}

impl Display for AnalyzerMeasurement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "\tAnalysis: {:>10?}", self.analysis);
        let _ = writeln!(f, "\t              ----------");
        let _ = writeln!(f, "\tTotal:        {:>10?}", self.total());

        Ok(())
    }
}
