use std::{
    borrow::Cow,
    collections::HashMap,
    hash::Hash,
    ptr,
    time::{Duration, Instant},
};

use hdrhistogram::Histogram;
use parking_lot::{Mutex, RwLock};
use tracing::{span, subscriber::Interest, Level, Metadata, Subscriber};
use tracing_subscriber::{layer::Context, prelude::*, registry::LookupSpan, Layer};

/// Implementation of a tracing [Layer] that collects timing information for spans into [Histogram]s
struct MetricsLayer;

lazy_static::lazy_static! {
    /// Global storage for metrics data
    static ref METRICS: RwLock<HashMap<CallsiteKey, Mutex<CallsiteEntry>>> = RwLock::default();
}

/// Static pointer to the metadata of a callsite, used as a unique identifier
/// for collecting spans created from there in the global metrics map
struct CallsiteKey(&'static Metadata<'static>);

impl PartialEq for CallsiteKey {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0, other.0)
    }
}

impl Eq for CallsiteKey {}

impl Hash for CallsiteKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(self.0, state);
    }
}

/// Single entry in the global callsite storage, containing handles to the
/// histograms associated with this callsite
enum CallsiteEntry {
    /// Spans with the debug level only count their total duration
    Debug { total: Histogram<u64> },
    /// Spans with the trace level count their total duration as well as
    /// individual busy and idle times
    Trace {
        total: Histogram<u64>,
        busy: Histogram<u64>,
        idle: Histogram<u64>,
    },
}

/// Extension data attached to tracing spans to keep track of their idle and busy time
///
/// Most of the associated code is based on the similar logic found in `tracing-subscriber`
/// for printing span timings to the console:
/// https://github.com/tokio-rs/tracing/blob/6f23c128fced6409008838a3223d76d7332d79e9/tracing-subscriber/src/fmt/fmt_subscriber.rs#L973
struct Timings {
    idle: u64,
    busy: u64,
    last: Instant,
}

impl Timings {
    fn new() -> Self {
        Self {
            idle: 0,
            busy: 0,
            last: Instant::now(),
        }
    }
}

impl<S> Layer<S> for MetricsLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    /// Only express interest in span callsites, disabling collection of events,
    /// and create new histogram for the spans created by this callsite
    fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest {
        if !metadata.is_span() {
            return Interest::never();
        }

        let entry = match metadata.level() {
            &Level::TRACE => CallsiteEntry::Trace {
                total: Histogram::new(3).unwrap(),
                busy: Histogram::new(3).unwrap(),
                idle: Histogram::new(3).unwrap(),
            },
            _ => CallsiteEntry::Debug {
                total: Histogram::new(3).unwrap(),
            },
        };

        METRICS
            .write()
            .insert(CallsiteKey(metadata), Mutex::new(entry));

        Interest::always()
    }

    /// When a new span is created, attach the timing data extension to it
    fn on_new_span(&self, _attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if extensions.get_mut::<Timings>().is_none() {
            extensions.insert(Timings::new());
        }
    }

    /// When a span is entered, start counting idle time for the parent span if
    /// it exists and busy time for the entered span itself
    fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");

        let now = Instant::now();
        if let Some(parent) = span.parent() {
            let mut extensions = parent.extensions_mut();
            if let Some(timings) = extensions.get_mut::<Timings>() {
                // The parent span was busy until now
                timings.busy += (now - timings.last).as_nanos() as u64;
                timings.last = now;
            }
        }

        let mut extensions = span.extensions_mut();
        if let Some(timings) = extensions.get_mut::<Timings>() {
            // The child span was idle until now
            timings.idle += (now - timings.last).as_nanos() as u64;
            timings.last = now;
        }
    }

    /// When a span is exited, stop it from counting busy time and start
    /// counting the parent as busy instead
    fn on_exit(&self, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");

        let now = Instant::now();
        let mut extensions = span.extensions_mut();
        if let Some(timings) = extensions.get_mut::<Timings>() {
            // Child span was busy until now
            timings.busy += (now - timings.last).as_nanos() as u64;
            timings.last = now;
        }

        // Re-enter parent
        if let Some(parent) = span.parent() {
            let mut extensions = parent.extensions_mut();
            if let Some(timings) = extensions.get_mut::<Timings>() {
                // Parent span was idle until now
                timings.idle += (now - timings.last).as_nanos() as u64;
                timings.last = now;
            }
        }
    }

    /// When a span is closed, extract its timing information and write it to
    /// the associated histograms
    fn on_close(&self, id: span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        let extensions = span.extensions();
        if let Some(timing) = extensions.get::<Timings>() {
            let Timings {
                busy: time_busy,
                idle: mut time_idle,
                last,
            } = *timing;

            // Count the time between the span being exited and it being closed as idle
            time_idle += (Instant::now() - last).as_nanos() as u64;

            // Acquire a read lock on the metrics storage, access the metrics entry
            // associated with this call site and acquire a write lock on it
            let metrics = METRICS.read();
            let entry = metrics.get(&CallsiteKey(span.metadata())).unwrap();
            let mut entry = entry.lock();

            match &mut *entry {
                CallsiteEntry::Debug { total } => {
                    total.record(time_busy + time_idle).unwrap();
                }
                CallsiteEntry::Trace { total, busy, idle } => {
                    busy.record(time_busy).unwrap();
                    idle.record(time_idle).unwrap();
                    total.record(time_busy + time_idle).unwrap();
                }
            }
        }
    }
}

/// Initializes metrics recording
pub fn init_metrics() {
    // Create and injects the metrics recording layer with the tracing library
    tracing_subscriber::registry().with(MetricsLayer).init();
}

/// Flush and print the recorded metrics to the console
pub fn print_metrics() {
    let mut histograms: Vec<_> = METRICS
        .write()
        .drain()
        .flat_map(|(key, entry)| {
            let name = key.0.name();
            match entry.into_inner() {
                CallsiteEntry::Debug { total } => vec![(Cow::Borrowed(name), total)],
                CallsiteEntry::Trace { total, busy, idle } => vec![
                    (Cow::Borrowed(name), total),
                    (Cow::Owned(format!("{name}.busy")), busy),
                    (Cow::Owned(format!("{name}.idle")), idle),
                ],
            }
        })
        .collect();

    histograms.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    for (key, histogram) in histograms {
        // Print the header line for the histogram with its name, mean sample
        // duration and standard deviation
        println!(
            "{}: mean = {:.1?}, stdev = {:.1?}",
            key,
            Duration::from_nanos(histogram.mean().round() as u64),
            Duration::from_nanos(histogram.stdev().round() as u64),
        );

        // For each quantile bucket in the histogram print out the associated
        // duration, a bar corresponding to the percentage of the total number
        // of samples falling within this bucket and the percentile
        // corresponding to this bucket
        let total = histogram.len() as f64;
        for v in histogram.iter_quantiles(1) {
            let duration = Duration::from_nanos(v.value_iterated_to());

            let count = v.count_since_last_iteration() as f64;
            let bar_length = (count * 40.0 / total).ceil() as usize;

            println!(
                "{: >7.1?} | {:40} | {:5.1}%",
                duration,
                "*".repeat(bar_length),
                v.quantile_iterated_to() * 100.0,
            );
        }

        // Print an empty line after each histogram
        println!();
    }
}
