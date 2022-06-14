use crate::BenchmarkSummary;
use criterion::black_box;
use rome_analyze::{analyze, AnalysisFilter, ControlFlow, Never};
use rome_js_syntax::JsAnyRoot;
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AnalyzerMeasurement {
    id: String,
    analysis: Duration,
}
pub fn benchmark_analyze_lib(id: &str, root: &JsAnyRoot) -> BenchmarkSummary {
    let analyzer_timer = timing::start();
    run_analyzer(root);
    let analyzer_duration = analyzer_timer.stop();

    BenchmarkSummary::Analyzer(AnalyzerMeasurement {
        id: id.to_string(),
        analysis: analyzer_duration,
    })
}

pub fn run_analyzer(root: &JsAnyRoot) {
    analyze(0, root, AnalysisFilter::default(), |event| {
        black_box(event.diagnostic());
        black_box(event.action());
        ControlFlow::<Never>::Continue(())
    });
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
