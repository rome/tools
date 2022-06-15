use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_diagnostics::file::FileSpan;
use rome_diagnostics::{file::FileId, Applicability, Severity};
use rome_diagnostics::{Diagnostic, DiagnosticTag, Footer, Span, SubDiagnostic};
use rome_rowan::{AstNode, Language, SyntaxNode, TextRange};

use crate::context::RuleContext;
use crate::{
    categories::{ActionCategory, RuleCategory},
    signals::{AnalyzerSignal, RuleSignal},
    ControlFlow,
};

/// The rule registry holds type-erased instances of all active analysis rules
pub struct RuleRegistry<L: Language> {
    rules: Vec<RegistryRule<L>>,
}

impl<L: Language> RuleRegistry<L> {
    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn push<R>(&mut self)
    where
        R: Rule + 'static,
        R::Query: AstNode<Language = L>,
    {
        self.rules.push(run::<R>);
    }
}

pub(crate) type RuleLanguage<R> = NodeLanguage<<R as Rule>::Query>;
pub(crate) type NodeLanguage<N> = <N as AstNode>::Language;

pub(crate) type RuleRoot<R> = LanguageRoot<RuleLanguage<R>>;
pub type LanguageRoot<L> = <L as Language>::Root;

impl<L> RuleRegistry<L>
where
    L: Language,
{
    // Run all rules known to the registry associated with nodes of type N
    pub(crate) fn analyze<B>(
        &self,
        file_id: FileId,
        root: &LanguageRoot<L>,
        node: SyntaxNode<L>,
        callback: &mut impl FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<B>,
    ) -> ControlFlow<B> {
        for rule in &self.rules {
            if let Some(event) = (rule)(file_id, root, &node) {
                if let ControlFlow::Break(b) = callback(&*event) {
                    return ControlFlow::Break(b);
                }
            }
        }

        ControlFlow::Continue(())
    }
}

/// Representation of a single rule in the registry as a generic function pointer
type RegistryRule<L> = for<'a> fn(
    FileId,
    &'a LanguageRoot<L>,
    &'a SyntaxNode<L>,
) -> Option<Box<dyn AnalyzerSignal<L> + 'a>>;

/// Generic implementation of RegistryRule for any rule type R
fn run<'a, R: Rule + 'static>(
    file_id: FileId,
    root: &'a RuleRoot<R>,
    node: &'a SyntaxNode<<R::Query as AstNode>::Language>,
) -> Option<Box<dyn AnalyzerSignal<RuleLanguage<R>> + 'a>> {
    if !<R::Query>::can_cast(node.kind()) {
        return None;
    }

    let query_result = <R::Query>::cast(node.clone())?;
    let ctx = RuleContext::new(query_result.clone(), root.clone());

    let result = R::run(&ctx)?;
    Some(RuleSignal::<R>::new_boxed(
        file_id,
        root,
        query_result,
        result,
    ))
}

/// Trait implemented by all analysis rules: declares interest to a certain AstNode type,
/// and a callback function to be executed on all nodes matching the query to possibly
/// raise an analysis event
pub trait Rule {
    /// The name of this rule, displayed in the diagnostics it emits
    const NAME: &'static str;
    /// The category this rule belong to, this is used for broadly filtering
    /// rules when running the analyzer
    const CATEGORY: RuleCategory;

    /// The type of AstNode this rule is interested in
    type Query: AstNode;
    /// A generic type that will be kept in memory between a call to `run` and
    /// subsequent executions of `diagnostic` or `action`, allows the rule to
    /// hold some temporary state between the moment a signal is raised and
    /// when a diagnostic or action needs to be built
    type State;

    /// This function is called once for each node matching `Query` in the tree
    /// being analyzed. If it returns `Some` the state object will be wrapped
    /// in a generic `AnalyzerSignal`, and the consumer of the analyzer may call
    /// `diagnostic` or `action` on it
    fn run(ctx: &RuleContext<Self>) -> Option<Self::State>;

    /// Called by the consumer of the analyzer to try to generate a diagnostic
    /// from a signal raised by `run`
    ///
    /// The default implementation returns None
    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        None
    }

    /// Called by the consumer of the analyzer to try to generate a code action
    /// from a signal raised by `run`
    ///
    /// The default implementation returns None
    fn action(
        _root: RuleRoot<Self>,
        _node: &Self::Query,
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
