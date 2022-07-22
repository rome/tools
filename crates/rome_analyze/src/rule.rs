use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_diagnostics::file::FileSpan;
use rome_diagnostics::{file::FileId, Applicability, Severity};
use rome_diagnostics::{Diagnostic, DiagnosticTag, Footer, Span, SubDiagnostic};
use rome_rowan::{Language, TextRange};

use crate::categories::{ActionCategory, RuleCategory};
use crate::context::RuleContext;
use crate::registry::RuleLanguage;
use crate::{AnalysisFilter, LanguageRoot, Phase, Phases, Queryable, RuleRegistry};

pub trait RuleMeta {
    /// It marks if a rule is deprecated, and if so a reason has to be provided.
    const DEPRECATED: Option<&'static str>;
    /// The version when the rule was implemented
    const VERSION: &'static str;
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
///     pub(crate) ExampleRule {
///         version: "0.7.0",
///         name: "ruleName"
///     }
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
///     pub(crate) NoVar {
///         version: "0.7.0",
///         name: "noVar"
///     }
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
///     pub(crate) NoVar {
///         version: "0.7.0",
///         name: "noVar"
///     }
/// }
/// ```
///
/// This will cause the documentation generator to ensure the rule does emit
/// exactly one diagnostic for this code, and to include a snapshot for the
/// diagnostic in the resulting documentation page
///
/// ## Deprecation
///
/// There are occasions when a rule must be deprecated, to avoid breaking changes. The reason
/// of deprecations can be multiples.
///
/// In order to to do, the macro allows to add additional field to add the reason for deprecation
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
///     pub(crate) NoVar {
///         version: "0.7.0",
///         name: "noVar",
///         deprecated: "Use the rule `noAnotherVar`"
///     }
/// }
///
#[macro_export]
macro_rules! declare_rule {
    ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident { version: $version:literal, name: $name:literal } ) => {
        $( #[doc = $doc] )*
        $vis enum $id {}

        impl $crate::RuleMeta for $id {
            const DEPRECATED: Option<&'static str> = None;
            const VERSION: &'static str = $version;
            const NAME: &'static str = $name;
            const DOCS: &'static str = concat!( $( $doc, "\n", )* );
        }
    };
    ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident { version: $version:literal, name: $name:literal, deprecated: $deprecated:literal, } ) => {
        $( #[doc = $doc] )*
        $vis enum $id {}

        impl $crate::RuleMeta for $id {
            const DEPRECATED: Option<&'static str> = Some($deprecated);
            const VERSION: &'static str = $version;
            const NAME: &'static str = $name;
            const DOCS: &'static str = concat!( $( $doc, "\n", )* );
        }
    };
}

/// A rule group is a collection of rules under a given name, serving as a
/// "namespace" for lint rules and allowing the entire set of rules to be
/// disabled at once
pub trait RuleGroup {
    type Language: Language;
    /// The name of this group, displayed in the diagnostics emitted by its rules
    const NAME: &'static str;
    /// Register all the rules belonging to this group into `registry` if they match `filter`
    fn push_rules(registry: &mut RuleRegistry<Self::Language>, filter: &AnalysisFilter);
}

/// This macro is used by the codegen script to declare an analyzer rule group,
/// and implement the [RuleGroup] trait for it
#[macro_export]
macro_rules! declare_group {
    ( $vis:vis $id:ident { name: $name:literal, rules: [ $( $rule:ty, )* ] } ) => {
        $vis enum $id {}

        impl $crate::RuleGroup for $id {
            type Language = <( $( $rule, )* ) as $crate::GroupLanguage>::Language;

            const NAME: &'static str = $name;

            fn push_rules(registry: &mut $crate::RuleRegistry<Self::Language>, filter: &$crate::AnalysisFilter) {
                $( if filter.match_rule::<Self, $rule>() { registry.push::<Self, $rule>(); } )*
            }
        }
    };
}

/// This trait is implemented for tuples of [Rule] types of size 1 to 20 if the
/// query type of all the rules in the tuple share the same associated
/// [Language] (which is then aliased as the `Language` associated type on
/// [GroupLanguage] itself). It is used to ensure all the rules in a given
/// group are all querying the same underlying language
pub trait GroupLanguage {
    type Language: Language;
}

/// Helper macro for implementing [GroupLanguage] on a large number of tuple types at once
macro_rules! impl_group_language {
    ( $head:ident $( , $rest:ident )* ) => {
        impl<$head $( , $rest )*> GroupLanguage for ($head, $( $rest ),*)
        where
            $head: Rule $( , $rest: Rule, <$rest as Rule>::Query: Queryable<Language = RuleLanguage<$head>> )*
        {
            type Language = RuleLanguage<$head>;
        }

        impl_group_language!( $( $rest ),* );
    };

    () => {};
}

impl_group_language!(
    T00, T01, T02, T03, T04, T05, T06, T07, T08, T09, T10, T11, T12, T13, T14, T15, T16, T17, T18,
    T19
);

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

    /// Used by the analyzer to associate a range of source text to a signal in
    /// order to support suppression comments.
    ///
    /// If this function returns [None], the range of the query node will be used instead
    ///
    /// The default implementation returns the range of `Self::diagnostic`, and
    /// should return the correct value for most rules however you may want to
    /// override this if generating a diagnostic for this rule requires heavy
    /// processing and the range could be determined through a faster path
    fn text_range(ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        Self::diagnostic(ctx, state).map(|diag| diag.span())
    }

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

    pub(crate) fn span(&self) -> TextRange {
        self.span
    }

    /// Convert this [`RuleDiagnostic`] into an instance of [`Diagnostic`] by
    /// injecting the name of the rule that emitted it and the ID of the file
    /// the rule was being run on
    pub(crate) fn into_diagnostic(
        self,
        file_id: FileId,
        code: String,
        code_link: String,
    ) -> Diagnostic {
        Diagnostic {
            file_id,
            severity: self.severity,
            code: Some(code),
            code_link: Some(code_link),
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
