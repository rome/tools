use crate::categories::{ActionCategory, RuleCategory};
use crate::context::RuleContext;
use crate::registry::{RuleLanguage, RuleSuppressions};
use crate::{AnalysisFilter, AnalyzerDiagnostic, Phase, Phases, Queryable, RuleRegistry};
use rome_console::fmt::Display;
use rome_console::{markup, MarkupBuf};
use rome_diagnostics::v2::Category;
use rome_diagnostics::{file::FileId, Applicability, Severity};
use rome_diagnostics::{DiagnosticTag, Footer, Span};
use rome_rowan::{BatchMutation, Language, TextRange};

/// Static metadata containing information about a rule
pub struct RuleMetadata {
    /// It marks if a rule is deprecated, and if so a reason has to be provided.
    pub deprecated: Option<&'static str>,
    /// The version when the rule was implemented
    pub version: &'static str,
    /// The name of this rule, displayed in the diagnostics it emits
    pub name: &'static str,
    /// The content of the documentation comments for this rule
    pub docs: &'static str,
    /// Whether a rule is recommended or not
    pub recommended: bool,
    /// The category this rule belong to, this is used for broadly filtering
    /// rules when running the analyzer
    pub category: RuleCategory,
}

impl RuleMetadata {
    pub const fn new(
        version: &'static str,
        name: &'static str,
        docs: &'static str,
        category: RuleCategory,
    ) -> Self {
        Self {
            deprecated: None,
            version,
            name,
            docs,
            recommended: false,
            category,
        }
    }

    pub const fn recommended(mut self, recommended: bool) -> Self {
        self.recommended = recommended;
        self
    }

    pub const fn deprecated(mut self, deprecated: &'static str) -> Self {
        self.deprecated = Some(deprecated);
        self
    }
}

pub trait RuleMeta {
    const METADATA: RuleMetadata;
}

/// This macro is used to declare an analyzer rule type, and implement the
//  [RuleMeta] trait for it
///  # Example
///
/// The macro itself expect the following syntax:
///
/// ```rust
///use rome_analyze::declare_rule;
///
/// declare_rule! {
///     /// Documentation
///     pub(crate) ExampleRule {
///         version: "0.7.0",
///         name: "ruleName",
///         recommended: false,
///     }
/// }
/// ```
///
/// Check [crate](module documentation) for a better
/// understanding of how the macro works
#[macro_export]
macro_rules! declare_rule {
    ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident {
        version: $version:literal,
        name: $name:tt,
        $( $key:ident: $value:expr, )*
    } ) => {
        $( #[doc = $doc] )*
        $vis enum $id {}

        impl $crate::RuleMeta for $id {
            const METADATA: $crate::RuleMetadata =
                $crate::RuleMetadata::new($version, $name, concat!( $( $doc, "\n", )* ), super::CATEGORY) $( .$key($value) )*;
        }

        // Declare a new `rule_category!` macro in the module context that
        // expands to the category of this rule
        // This is implemented by calling the `group_category!` macro from the
        // parent module (that should be declared by a call to `declare_group!`)
        // and providing it with the name of this rule as a string literal token
        #[allow(unused_macros)]
        macro_rules! rule_category {
            () => { super::group_category!( $name ) };
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
    ( $vis:vis $id:ident { name: $name:tt, rules: [ $( $( $rule:ident )::* , )* ] } ) => {
        $vis enum $id {}

        impl $crate::RuleGroup for $id {
            type Language = <( $( $( $rule )::* , )* ) as $crate::GroupLanguage>::Language;

            const NAME: &'static str = $name;

            fn push_rules(registry: &mut $crate::RuleRegistry<Self::Language>, filter: &$crate::AnalysisFilter) {
                $( if filter.match_rule::<Self, $( $rule )::*>() { registry.push::<Self, $( $rule )::*>(); } )*
            }
        }

        pub(self) use super::CATEGORY;

        // Declare a `group_category!` macro in the context of this module (and
        // all its children). This macro takes the name of a rule as a string
        // literal token and expands to the category of the lint rule with this
        // name within this group.
        // This is implemented by calling the `category_concat!` macro with the
        // "lint" prefix, the name of this group, and the rule name argument
        #[allow(unused_macros)]
        macro_rules! group_category {
            ( $rule_name:tt ) => { $crate::category_concat!( "lint", $name, $rule_name ) };
        }

        // Re-export the macro for child modules, so `declare_rule!` can access
        // the category of its parent group by using the `super` module
        pub(self) use group_category;
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
    T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29
);

/// Trait implemented by all analysis rules: declares interest to a certain AstNode type,
/// and a callback function to be executed on all nodes matching the query to possibly
/// raise an analysis event
pub trait Rule: RuleMeta {
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

    /// Allows the rule to suppress a set of syntax nodes to prevent them from
    /// matching the `Query`. This is useful for rules that implement a code
    /// action that recursively modifies multiple nodes at once, this hook
    /// allows these rules to avoid matching on those nodes again.
    ///
    /// # Example
    ///
    /// ```ignore
    /// impl Rule for SimplifyExpression {
    ///     type Query = BinaryExpression;
    ///
    ///     fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    ///         // Recursively check this expression and its children for simplification
    ///         // opportunities
    ///         check_can_simplify(ctx.query())
    ///     }
    ///
    ///     fn suppressed_nodes(
    ///         _ctx: &RuleContext<Self>,
    ///         state: &Self::State,
    ///         suppressions: &mut RuleSuppressions<RuleLanguage<Self>>
    ///     ) {
    ///         // Prevent this rule from matching again on nodes that were already checked by
    ///         // `check_can_simplify`
    ///         for node in &state.nodes {
    ///             suppressions.suppress_node(node.clone());
    ///         }
    ///     }
    /// }
    /// ```
    fn suppressed_nodes(
        ctx: &RuleContext<Self>,
        state: &Self::State,
        suppressions: &mut RuleSuppressions<RuleLanguage<Self>>,
    ) {
        let (..) = (ctx, state, suppressions);
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
        ctx: &RuleContext<Self>,
        state: &Self::State,
    ) -> Option<RuleAction<RuleLanguage<Self>>> {
        let (..) = (ctx, state);
        None
    }
}

/// Diagnostic object returned by a single analysis rule
pub struct RuleDiagnostic {
    pub(crate) category: &'static Category,
    pub(crate) span: TextRange,
    pub(crate) title: MarkupBuf,
    pub(crate) summary: Option<String>,
    pub(crate) tag: Option<DiagnosticTag>,
    pub(crate) primary: Option<MarkupBuf>,
    pub(crate) secondaries: Vec<(Severity, MarkupBuf, TextRange)>,
    pub(crate) footers: Vec<Footer>,
}

// Some of these methods aren't used by anything yet
#[allow(dead_code)]
impl RuleDiagnostic {
    /// Creates a new [`RuleDiagnostic`] with a severity and title that will be
    /// used in a builder-like way to modify labels.
    pub fn new(category: &'static Category, span: impl Span, title: impl Display) -> Self {
        Self {
            category,
            span: span.as_range(),
            title: markup!({ title }).to_owned(),
            summary: None,
            tag: None,
            primary: None,
            secondaries: Vec::new(),
            footers: Vec::new(),
        }
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

    /// Convert this [`RuleDiagnostic`] into an instance of [`AnalyzerDiagnostic`] by
    /// injecting the name of the rule that emitted it and the ID of the file
    /// the rule was being run on
    pub(crate) fn into_analyzer_diagnostic(self, file_id: FileId) -> AnalyzerDiagnostic {
        AnalyzerDiagnostic::from_rule_diagnostic(file_id, self)
    }
}

/// Code Action object returned by a single analysis rule
pub struct RuleAction<L: Language> {
    pub category: ActionCategory,
    pub applicability: Applicability,
    pub message: MarkupBuf,
    pub mutation: BatchMutation<L>,
}
