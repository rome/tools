use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_diagnostics::file::FileSpan;
use rome_diagnostics::{file::FileId, Applicability, Severity};
use rome_diagnostics::{Diagnostic, DiagnosticTag, Footer, Span, SubDiagnostic};
use rome_rowan::{Language, TextRange};

use crate::categories::{ActionCategory, RuleCategory};
use crate::context::RuleContext;
use crate::registry::RuleLanguage;
use crate::{LanguageRoot, Phase, Phases, Queryable};

pub trait RuleMeta {
    /// The name of this rule, displayed in the diagnostics it emits
    const NAME: &'static str;
    /// The content of the documentation comments for this rule
    const DOCS: &'static str;
}

/// This macro is used to declare an analyzer rule type, and implement the
/// [RuleMeta] trait for it
///
/// # Example
///
/// The macro itself expect the following syntax:
/// ```ignore
/// declare_rule! {
///     /// Documentation
///     pub(crate) ExampleRule = "ruleName"
/// }
/// ```
///
/// # Documentation
///
/// The doc-comment for the rule is mandatory and is used to generate the
/// documentation page for the rule on the website.
///
/// Importantly, the tool used to generate those pages also runs tests on the
/// code blocks included in the documentation written in languages supported by
/// the Rome toolchain (JavaScript, JSX, TypeScript, ...) similar to how
/// `rustdoc` generates tests from code blocks written in Rust. Because code
/// blocks in Rust doc-comments are assumed to be written in Rust by default
/// the language of the test must be explicitly specified, for instance:
///
/// ```ignore
/// declare_rule! {
///     /// Disallow the use of `var`
///     ///
///     /// ### Valid
///     ///
///     /// ```js
///     /// let a, b;
///     /// ```
///     pub(crate) NoVar = "noVar"
/// }
/// ```
///
/// Additionally, it's possible to declare that a test should emit a diagnostic
/// by adding `expect_diagnostic` to the language metadata:
///
/// ```ignore
/// declare_rule! {
///     /// Disallow the use of `var`
///     ///
///     /// ### Invalid
///     ///
///     /// ```js,expect_diagnostic
///     /// var a, b;
///     /// ```
///     pub(crate) NoVar = "noVar"
/// }
/// ```
///
/// This will cause the documentation generator to ensure the rule does emit
/// exactly one diagnostic for this code, and to include a snapshot for the
/// diagnostic in the resulting documentation page
#[macro_export]
macro_rules! declare_rule {
    ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident = $name:literal ) => {
        $( #[doc = $doc] )*
        $vis enum $id {}

        impl $crate::RuleMeta for $id {
            const NAME: &'static str = $name;
            const DOCS: &'static str = concat!( $( $doc, "\n", )* );
        }
    };
}

/// Trait implemented by all analysis rules: declares interest to a certain AstNode type,
/// and a callback function to be executed on all nodes matching the query to possibly
/// raise an analysis event
pub trait Rule: RuleMeta {
    /// The category this rule belong to, this is used for broadly filtering
    /// rules when running the analyzer
    const CATEGORY: RuleCategory;

    /// The type of AstNode this rule is interested in
    type Query: Queryable;
    /// A generic type that will be kept in memory between a call to `run` and
    /// subsequent executions of `diagnostic` or `action`, allows the rule to
    /// hold some temporary state between the moment a signal is raised and
    /// when a diagnostic or action needs to be built
    type State;
    /// An iterator type returned by `run` to yield zero or more signals to the
    /// analyzer
    type Signals: IntoIterator<Item = Self::State>;

    fn phase() -> Phases {
        <<<Self as Rule>::Query as Queryable>::Services as Phase>::phase()
    }

    /// This function is called once for each node matching `Query` in the tree
    /// being analyzed. If it returns `Some` the state object will be wrapped
    /// in a generic `AnalyzerSignal`, and the consumer of the analyzer may call
    /// `diagnostic` or `action` on it
    fn run(ctx: &RuleContext<Self>) -> Self::Signals;

    /// Called by the consumer of the analyzer to try to generate a diagnostic
    /// from a signal raised by `run`
    ///
    /// The default implementation returns None
    fn diagnostic(_ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        None
    }

    /// Called by the consumer of the analyzer to try to generate a code action
    /// from a signal raised by `run`
    ///
    /// The default implementation returns None
    fn action(
        _ctx: &RuleContext<Self>,
        _state: &Self::State,
    ) -> Option<RuleAction<RuleLanguage<Self>>> {
        None
    }
}

/// Diagnostic object returned by a single analysis rule
pub struct RuleDiagnostic {
    severity: Severity,
    span: TextRange,
    title: MarkupBuf,
    summary: Option<String>,
    tag: Option<DiagnosticTag>,
    primary: Option<MarkupBuf>,
    secondaries: Vec<(Severity, MarkupBuf, TextRange)>,
    footers: Vec<Footer>,
}

// Some of these methods aren't used by anything yet
#[allow(dead_code)]
impl RuleDiagnostic {
    /// Creates a new [`RuleDiagnostic`] with a severity and title that will be
    /// used in a builder-like way to modify labels.
    fn new(severity: Severity, span: impl Span, title: impl Display) -> Self {
        Self {
            severity,
            span: span.as_range(),
            title: markup!({ title }).to_owned(),
            summary: None,
            tag: None,
            primary: None,
            secondaries: Vec::new(),
            footers: Vec::new(),
        }
    }

    /// Creates a new [`RuleDiagnostic`] with the `Error` severity.
    pub fn error(span: impl Span, title: impl Display) -> Self {
        Self::new(Severity::Error, span, title)
    }

    /// Creates a new [`RuleDiagnostic`] with the `Warning` severity.
    pub fn warning(span: impl Span, title: impl Display) -> Self {
        Self::new(Severity::Warning, span, title)
    }

    /// Creates a new [`RuleDiagnostic`] with the `Help` severity.
    pub fn help(span: impl Span, title: impl Display) -> Self {
        Self::new(Severity::Help, span, title)
    }

    /// Creates a new [`RuleDiagnostic`] with the `Note` severity.
    pub fn note(span: impl Span, title: impl Display) -> Self {
        Self::new(Severity::Note, span, title)
    }

    /// Set an explicit plain-text summary for this diagnostic.
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Marks this diagnostic as deprecated code, which will
    /// be displayed in the language server.
    ///
    /// This does not have any influence on the diagnostic rendering.
    pub fn deprecated(mut self) -> Self {
        self.tag = if matches!(self.tag, Some(DiagnosticTag::Unnecessary)) {
            Some(DiagnosticTag::Both)
        } else {
            Some(DiagnosticTag::Deprecated)
        };
        self
    }

    /// Marks this diagnostic as unnecessary code, which will
    /// be displayed in the language server.
    ///
    /// This does not have any influence on the diagnostic rendering.
    pub fn unnecessary(mut self) -> Self {
        self.tag = if matches!(self.tag, Some(DiagnosticTag::Deprecated)) {
            Some(DiagnosticTag::Both)
        } else {
            Some(DiagnosticTag::Unnecessary)
        };
        self
    }

    /// Attaches a label to this [`RuleDiagnostic`], that will point to another file
    /// that is provided.
    pub fn label_in_file(mut self, severity: Severity, span: impl Span, msg: impl Display) -> Self {
        self.secondaries
            .push((severity, markup!({ msg }).to_owned(), span.as_range()));
        self
    }

    /// Attaches a label to this [`RuleDiagnostic`].
    ///
    /// The given span has to be in the file that was provided while creating this [`RuleDiagnostic`].
    pub fn label(mut self, severity: Severity, span: impl Span, msg: impl Display) -> Self {
        self.secondaries
            .push((severity, markup!({ msg }).to_owned(), span.as_range()));
        self
    }

    /// Attaches a primary label to this [`RuleDiagnostic`].
    pub fn primary(mut self, msg: impl Display) -> Self {
        self.primary = Some(markup!({ msg }).to_owned());
        self
    }

    /// Attaches a secondary label to this [`RuleDiagnostic`].
    pub fn secondary(self, span: impl Span, msg: impl Display) -> Self {
        self.label(Severity::Note, span, msg)
    }

    /// Adds a footer to this [`RuleDiagnostic`], which will be displayed under the actual error.
    pub fn footer(mut self, severity: Severity, msg: impl Display) -> Self {
        self.footers.push(Footer {
            msg: markup!({ msg }).to_owned(),
            severity,
        });
        self
    }

    /// Adds a footer to this [`RuleDiagnostic`], with the `Help` severity.
    pub fn footer_help(self, msg: impl Display) -> Self {
        self.footer(Severity::Help, msg)
    }

    /// Adds a footer to this [`RuleDiagnostic`], with the `Note` severity.
    pub fn footer_note(self, msg: impl Display) -> Self {
        self.footer(Severity::Note, msg)
    }

    /// Convert this [`RuleDiagnostic`] into an instance of [`Diagnostic`] by
    /// injecting the name of the rule that emitted it and the ID of the file
    /// the rule was being run on
    pub(crate) fn into_diagnostic(self, file_id: FileId, code: &'static str) -> Diagnostic {
        Diagnostic {
            file_id,
            severity: self.severity,
            code: Some(code.into()),
            title: self.title,
            summary: self.summary,
            tag: self.tag,
            primary: Some(SubDiagnostic {
                severity: self.severity,
                msg: self.primary.unwrap_or_default(),
                span: FileSpan {
                    file: file_id,
                    range: self.span,
                },
            }),
            children: self
                .secondaries
                .into_iter()
                .map(|(severity, msg, range)| SubDiagnostic {
                    severity,
                    msg,
                    span: FileSpan {
                        file: file_id,
                        range,
                    },
                })
                .collect(),
            suggestions: Vec::new(),
            footers: self.footers,
        }
    }
}

/// Code Action object returned by a single analysis rule
pub struct RuleAction<L: Language> {
    pub category: ActionCategory,
    pub applicability: Applicability,
    pub message: MarkupBuf,
    pub root: LanguageRoot<L>,
}
