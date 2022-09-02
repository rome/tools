use crate::prelude::*;
use rome_formatter::{
    write, FormatContext, FormatOwnedWithRule, FormatRefWithRule, FormatRuleWithOptions,
};

use crate::{AsFormat, IntoFormat};
use rome_js_syntax::{
    JsAnyExpression, JsAssignmentExpression, JsCallExpression, JsComputedMemberExpression,
    JsConditionalExpression, JsInitializerClause, JsNewExpression, JsReturnStatement,
    JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, JsThrowStatement,
    JsUnaryExpression, JsYieldArgument, SourceType, TsAsExpression, TsConditionalType,
    TsNonNullAssertionExpression, TsType,
};
use rome_rowan::{declare_node_union, match_ast, AstNode, SyntaxResult};

declare_node_union! {
    pub JsAnyConditional = JsConditionalExpression | TsConditionalType
}

impl<'a> AsFormat<'a> for JsAnyConditional {
    type Format = FormatRefWithRule<'a, JsAnyConditional, FormatJsAnyConditionalRule>;

    fn format(&'a self) -> Self::Format {
        FormatRefWithRule::new(self, FormatJsAnyConditionalRule::default())
    }
}

impl IntoFormat<JsFormatContext> for JsAnyConditional {
    type Format = FormatOwnedWithRule<JsAnyConditional, FormatJsAnyConditionalRule>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatJsAnyConditionalRule::default())
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatJsAnyConditionalRule {
    /// Whether the parent is a jsx conditional chain.
    /// Gets passed through from the root to the consequent and alternate of [JsConditionalExpression]s.
    ///
    /// Doesn't apply for [TsConditionalType].
    jsx_chain: ConditionalJsxChain,
}

impl FormatRuleWithOptions<JsAnyConditional> for FormatJsAnyConditionalRule {
    type Options = ConditionalJsxChain;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.jsx_chain = options;
        self
    }
}

impl FormatRule<JsAnyConditional> for FormatJsAnyConditionalRule {
    type Context = JsFormatContext;

    fn fmt(
        &self,
        conditional: &JsAnyConditional,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        let syntax = conditional.syntax();
        let consequent = conditional.consequent()?;
        let alternate = conditional.alternate()?;

        let layout = self.layout(conditional, f.context().options().source_type());
        let jsx_chain = layout.jsx_chain().unwrap_or(self.jsx_chain);

        let format_consequent_and_alternate = format_with(|f| {
            write!(
                f,
                [
                    soft_line_break_or_space(),
                    conditional.question_mark_token().format(),
                    space()
                ]
            )?;

            let is_consequent_nested = consequent.syntax().kind() == syntax.kind();

            if is_consequent_nested {
                // Add parentheses around the consequent if it is a conditional expression and fits on the same line
                // so that it's easier to identify the parts that belong to a conditional expression.
                // `a ? b ? c: d : e` -> `a ? (b ? c: d) : e
                write!(
                    f,
                    [
                        if_group_fits_on_line(&text("(")),
                        align(2, &consequent),
                        if_group_fits_on_line(&text(")"))
                    ]
                )?;
            } else {
                write!(f, [align(2, &consequent)])?;
            }

            write!(
                f,
                [
                    soft_line_break_or_space(),
                    conditional.colon_token().format(),
                    space()
                ]
            )?;

            let is_alternate_nested = alternate.syntax().kind() == syntax.kind();

            // Don't "indent" a nested alternate by two spaces so that the consequent and alternates nicely align
            // ```
            // // Prefer **this** over                          // **this**
            // aLongLongLongLong                                 aLongLongLongLong
            // 	? bLongLongLongLong                             ? bLongLongLongLong
            // 	: cLongLongLong                                 : cLongLongLong
            // 	? dLongLongLong                                   ? dLongLong
            // 	: eLongLongLong;                                  : eLongLongLong
            // ```
            if is_alternate_nested {
                write!(f, [alternate])
            } else {
                write!(f, [align(2, &alternate)])
            }
        });

        let format_tail_with_indent = format_with(|f: &mut JsFormatter| {
            match conditional {
                JsAnyConditional::JsConditionalExpression(conditional) if jsx_chain.is_chain() => {
                    write!(
                        f,
                        [
                            space(),
                            conditional.question_mark_token().format(),
                            space(),
                            format_jsx_chain_consequent(consequent.as_expression().unwrap()),
                            space(),
                            conditional.colon_token().format(),
                            space(),
                            format_jsx_chain_alternate(alternate.as_expression().unwrap())
                        ]
                    )
                }
                _ => {
                    // Add an extra level of indent to nested consequences.
                    if layout.is_nested_consequent() {
                        // if f.context().indent_style().is_tab() {
                        // This may look silly but the `dedent` is to remove the outer `align` added by the parent's formatting of the consequent.
                        // The `indent` is necessary to indent the content by one level with a tab.
                        // Adding an `indent` without the `dedent` would result in the `outer` align being converted
                        // into a `indent` + the `indent` added here, ultimately resulting in a two-level indention.
                        write!(f, [dedent(&indent(&format_consequent_and_alternate))])
                    } else {
                        format_consequent_and_alternate.fmt(f)
                    }
                }
            }
        });

        let should_extra_indent = self.should_extra_indent(conditional, &layout);

        let format_inner = format_with(|f| {
            write!(
                f,
                [FormatConditionalTest {
                    conditional,
                    layout: &layout,
                }]
            )?;

            // Indent the `consequent` and `alternate` **only if** this is the root conditional expression
            // OR this is the `test` of a conditional expression.
            if jsx_chain.is_no_chain() && (layout.is_root() || layout.is_nested_test()) {
                write!(f, [indent(&format_tail_with_indent)])?;
            } else {
                // Don't indent for nested `alternate`s or `consequence`s
                write!(f, [format_tail_with_indent])?;
            }

            let break_closing_parentheses = jsx_chain.is_no_chain()
                && self.is_parent_static_member_expression(conditional, &layout);

            // Add a soft line break in front of the closing `)` in case the parent is a static member expression
            // ```
            // (veryLongCondition
            //      ? a
            //      : b // <- enforce line break here if the conditional breaks
            // ).more
            // ```
            if break_closing_parentheses && !should_extra_indent {
                write!(f, [soft_line_break()])?;
            }

            // Make sure that line suffix comments to not escape
            write!(f, [line_suffix_boundary()])
        });

        let grouped = format_with(|f| {
            if layout.is_root() {
                group(&format_inner).fmt(f)
            } else {
                format_inner.fmt(f)
            }
        });

        if layout.is_nested_test() || should_extra_indent {
            group(&soft_block_indent(&grouped)).fmt(f)
        } else {
            grouped.fmt(f)
        }
    }
}

impl FormatJsAnyConditionalRule {
    fn layout(&self, conditional: &JsAnyConditional, source_type: SourceType) -> ConditionalLayout {
        match conditional.syntax().parent() {
            Some(parent) if parent.kind() == conditional.syntax().kind() => {
                let conditional_parent = JsAnyConditional::unwrap_cast(parent);

                if conditional_parent.is_test(conditional.syntax()) {
                    ConditionalLayout::NestedTest {
                        parent: conditional_parent,
                    }
                } else if conditional_parent.is_alternate(conditional.syntax()) {
                    ConditionalLayout::NestedAlternate {
                        parent: conditional_parent,
                    }
                } else {
                    ConditionalLayout::NestedConsequent {
                        parent: conditional_parent,
                    }
                }
            }
            parent => {
                let is_jsx_chain = match conditional {
                    JsAnyConditional::JsConditionalExpression(conditional)
                        if source_type.variant().is_jsx() =>
                    {
                        is_jsx_conditional_chain(conditional)
                    }
                    _ => false,
                };

                ConditionalLayout::Root {
                    parent,
                    jsx_chain: is_jsx_chain.into(),
                }
            }
        }
    }

    /// It is desired to add an extra indent if this conditional is a [JsConditionalExpression] and is directly inside
    /// of a member chain:
    ///
    /// ```javascript
    /// // Input
    /// return (a ? b : c).member
    ///
    /// // Default
    /// return (a
    ///     ? b
    ///     : c
    /// ).member
    ///
    /// // Preferred
    /// return (
    ///     a
    ///         ? b
    ///         : c
    /// ).member
    /// ```
    fn should_extra_indent(
        &self,
        conditional: &JsAnyConditional,
        layout: &ConditionalLayout,
    ) -> bool {
        enum Ancestor {
            MemberChain(JsAnyExpression),
            Root(JsSyntaxNode),
        }

        let conditional = match conditional {
            JsAnyConditional::JsConditionalExpression(conditional) => conditional,
            JsAnyConditional::TsConditionalType(_) => {
                return false;
            }
        };

        let ancestors = layout
            .parent()
            .into_iter()
            .flat_map(|parent| parent.ancestors());
        let mut parent = None;
        let mut expression = JsAnyExpression::from(conditional.clone());

        // This tries to find the start of a member chain by iterating over all ancestors of the conditional.
        // The iteration "breaks" as soon as a non-member-chain node is found.
        for ancestor in ancestors {
            let ancestor = match_ast! {
                match &ancestor {
                    JsCallExpression(call_expression) => {
                        if call_expression
                            .callee()
                            .as_ref()
                            == Ok(&expression)
                        {
                            Ancestor::MemberChain(call_expression.into())
                        } else {
                            Ancestor::Root(call_expression.into_syntax())
                        }
                    },

                    JsStaticMemberExpression(member_expression) => {
                        if member_expression
                            .object()
                            .as_ref()
                            == Ok(&expression)
                        {
                            Ancestor::MemberChain(member_expression.into())
                        } else {
                            Ancestor::Root(member_expression.into_syntax())
                        }
                    },
                    JsComputedMemberExpression(member_expression) => {
                        if member_expression
                            .object()
                            .as_ref()
                            == Ok(&expression)
                        {
                            Ancestor::MemberChain(member_expression.into())
                        } else {
                            Ancestor::Root(member_expression.into_syntax())
                        }
                    },
                    TsNonNullAssertionExpression(non_null_assertion) => {
                        if non_null_assertion
                            .expression()
                            .as_ref()
                            == Ok(&expression)
                        {
                            Ancestor::MemberChain(non_null_assertion.into())
                        } else {
                            Ancestor::Root(non_null_assertion.into_syntax())
                        }
                    },
                    JsNewExpression(new_expression) => {
                        // Skip over new expressions
                        if new_expression
                            .callee()
                            .as_ref()
                            == Ok(&expression)
                        {
                            parent = new_expression.syntax().parent();
                            expression = new_expression.into();
                            break;
                        }

                        Ancestor::Root(new_expression.into_syntax())
                    },
                    TsAsExpression(as_expression) => {
                        if as_expression
                            .expression()
                            .as_ref()
                            == Ok(&expression)
                        {
                            parent = as_expression.syntax().parent();
                            expression = as_expression.into();
                            break;
                        }

                        Ancestor::Root(as_expression.into_syntax())
                    },
                    _ => Ancestor::Root(ancestor),
                }
            };

            match ancestor {
                Ancestor::MemberChain(left) => {
                    // Store the node that is highest in the member chain
                    expression = left;
                }
                Ancestor::Root(root) => {
                    parent = Some(root);
                    break;
                }
            }
        }

        // Don't indent if this conditional isn't part of a member chain.
        // e.g. don't indent for `return a ? b : c`, only for `return (a ? b : c).member`
        if expression.syntax() == conditional.syntax() {
            return false;
        }

        match parent {
            None => false,
            Some(parent) => {
                let argument = match parent.kind() {
                    JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                        let initializer = JsInitializerClause::unwrap_cast(parent);
                        initializer.expression().ok().map(JsAnyExpression::from)
                    }
                    JsSyntaxKind::JS_RETURN_STATEMENT => {
                        let return_statement = JsReturnStatement::unwrap_cast(parent);
                        return_statement.argument().map(JsAnyExpression::from)
                    }
                    JsSyntaxKind::JS_THROW_STATEMENT => {
                        let throw_statement = JsThrowStatement::unwrap_cast(parent);
                        throw_statement.argument().ok().map(JsAnyExpression::from)
                    }
                    JsSyntaxKind::JS_UNARY_EXPRESSION => {
                        let unary_expression = JsUnaryExpression::unwrap_cast(parent);
                        unary_expression.argument().ok().map(JsAnyExpression::from)
                    }
                    JsSyntaxKind::JS_YIELD_ARGUMENT => {
                        let yield_argument = JsYieldArgument::unwrap_cast(parent);
                        yield_argument.expression().ok().map(JsAnyExpression::from)
                    }
                    JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                        let assignment_expression = JsAssignmentExpression::unwrap_cast(parent);
                        assignment_expression
                            .right()
                            .ok()
                            .map(JsAnyExpression::from)
                    }
                    _ => None,
                };

                argument.map_or(false, |argument| argument == expression)
            }
        }
    }

    /// Returns `true` if this is the root conditional expression and the parent is a [JsStaticMemberExpression].
    fn is_parent_static_member_expression(
        &self,
        conditional: &JsAnyConditional,
        layout: &ConditionalLayout,
    ) -> bool {
        if !conditional.is_conditional_expression() {
            return false;
        }

        match layout {
            ConditionalLayout::Root {
                parent: Some(parent),
                ..
            } => JsStaticMemberExpression::can_cast(parent.kind()),
            _ => false,
        }
    }
}

/// Formats the test conditional of a conditional expression.
struct FormatConditionalTest<'a> {
    conditional: &'a JsAnyConditional,
    layout: &'a ConditionalLayout,
}

impl Format<JsFormatContext> for FormatConditionalTest<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let format_inner = format_with(|f| match self.conditional {
            JsAnyConditional::JsConditionalExpression(conditional) => {
                write!(f, [conditional.test().format()])
            }
            JsAnyConditional::TsConditionalType(conditional) => {
                write!(
                    f,
                    [
                        conditional.check_type().format(),
                        space(),
                        conditional.extends_token().format(),
                        space(),
                        conditional.extends_type().format()
                    ]
                )
            }
        });

        if self.layout.is_nested_alternate() {
            align(2, &format_inner).fmt(f)
        } else {
            format_inner.fmt(f)
        }
    }
}

declare_node_union! {
    ExpressionOrType = JsAnyExpression | TsType
}

impl ExpressionOrType {
    fn as_expression(&self) -> Option<&JsAnyExpression> {
        match self {
            ExpressionOrType::JsAnyExpression(expression) => Some(expression),
            ExpressionOrType::TsType(_) => None,
        }
    }
}

impl Format<JsFormatContext> for ExpressionOrType {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            ExpressionOrType::JsAnyExpression(expression) => expression.format().fmt(f),
            ExpressionOrType::TsType(ty) => ty.format().fmt(f),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum ConditionalLayout {
    /// Conditional that is the `alternate` of another conditional.
    ///
    /// The `test` condition of a nested alternated is aligned with the parent's `:`.
    ///
    /// ```javascript
    /// outerCondition
    /// 	? consequent
    /// 	: nestedAlternate +
    /// 	  binary + // <- notice how the content is aligned to the `: `
    /// 	? consequentOfnestedAlternate
    /// 	: alternateOfNestedAlternate;
    /// ```
    NestedAlternate { parent: JsAnyConditional },

    /// Conditional that is the `test` of another conditional.
    ///
    /// ```javascript
    /// (
    ///     a              // <-- Note the extra indent here
    ///         ? b
    ///         : c
    ///  )
    ///     ? d
    ///     : e;
    /// ```
    ///
    /// Indents the
    NestedTest { parent: JsAnyConditional },

    /// Conditional that is the `consequent` of another conditional.
    ///
    /// ```javascript
    /// condition1
    /// 	? condition2
    /// 		? consequent2 // <-- consequent and alternate gets indented
    /// 		: alternate2
    /// 	: alternate1;
    /// ```
    NestedConsequent { parent: JsAnyConditional },

    /// This conditional isn't a child of another conditional.
    ///
    /// ```javascript
    /// return a ? b : c;
    /// ```
    Root {
        /// The closest ancestor that isn't a parenthesized node.
        parent: Option<JsSyntaxNode>,

        jsx_chain: ConditionalJsxChain,
    },
}

/// A [JsConditionalExpression] that itself or any of its parent's [JsConditionalExpression] have a a [JsxTagExpression]
/// as its [`test`](JsConditionalExpression::test), [`consequent`](JsConditionalExpression::consequent) or [`alternate`](JsConditionalExpression::alternate).
///
/// Parenthesizes the `consequent` and `alternate` if it the group breaks except if the expressions are
/// * `null`
/// * `undefined`
/// * or a nested [JsConditionalExpression] in the alternate branch
///
/// ```javascript
/// abcdefgh? (
///   <Element>
///     <Sub />
///     <Sub />
///   </Element>
/// ) : (
///   <Element2>
///     <Sub />
///     <Sub />
///   </Element2>
/// );
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum ConditionalJsxChain {
    Chain,
    #[default]
    NoChain,
}

impl ConditionalJsxChain {
    pub const fn is_chain(&self) -> bool {
        matches!(self, ConditionalJsxChain::Chain)
    }
    pub const fn is_no_chain(&self) -> bool {
        matches!(self, ConditionalJsxChain::NoChain)
    }
}

impl From<bool> for ConditionalJsxChain {
    fn from(value: bool) -> Self {
        match value {
            true => ConditionalJsxChain::Chain,
            false => ConditionalJsxChain::NoChain,
        }
    }
}

impl ConditionalLayout {
    const fn jsx_chain(&self) -> Option<ConditionalJsxChain> {
        match self {
            ConditionalLayout::NestedAlternate { .. }
            | ConditionalLayout::NestedTest { .. }
            | ConditionalLayout::NestedConsequent { .. } => None,
            ConditionalLayout::Root { jsx_chain, .. } => Some(*jsx_chain),
        }
    }

    const fn is_root(&self) -> bool {
        matches!(self, ConditionalLayout::Root { .. })
    }

    /// Returns the parent node, if any
    fn parent(&self) -> Option<&JsSyntaxNode> {
        match self {
            ConditionalLayout::NestedAlternate { parent, .. }
            | ConditionalLayout::NestedTest { parent, .. }
            | ConditionalLayout::NestedConsequent { parent, .. } => Some(parent.syntax()),
            ConditionalLayout::Root { parent, .. } => parent.as_ref(),
        }
    }

    const fn is_nested_test(&self) -> bool {
        matches!(self, ConditionalLayout::NestedTest { .. })
    }

    const fn is_nested_alternate(&self) -> bool {
        matches!(self, ConditionalLayout::NestedAlternate { .. })
    }

    const fn is_nested_consequent(&self) -> bool {
        matches!(self, ConditionalLayout::NestedConsequent { .. })
    }
}

impl JsAnyConditional {
    /// Returns `true` if `node` is the `test` of this conditional.
    fn is_test(&self, node: &JsSyntaxNode) -> bool {
        match self {
            JsAnyConditional::JsConditionalExpression(conditional) => conditional
                .test()
                .ok()
                .map_or(false, |resolved| resolved.syntax() == node),
            JsAnyConditional::TsConditionalType(conditional) => {
                conditional.check_type().map(AstNode::into_syntax).as_ref() == Ok(node)
                    || conditional
                        .extends_type()
                        .map(AstNode::into_syntax)
                        .as_ref()
                        == Ok(node)
            }
        }
    }

    fn question_mark_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyConditional::JsConditionalExpression(conditional) => {
                conditional.question_mark_token()
            }
            JsAnyConditional::TsConditionalType(conditional) => conditional.question_mark_token(),
        }
    }

    fn consequent(&self) -> SyntaxResult<ExpressionOrType> {
        match self {
            JsAnyConditional::JsConditionalExpression(conditional) => {
                conditional.consequent().map(ExpressionOrType::from)
            }
            JsAnyConditional::TsConditionalType(conditional) => {
                conditional.true_type().map(ExpressionOrType::from)
            }
        }
    }

    fn colon_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyConditional::JsConditionalExpression(conditional) => conditional.colon_token(),
            JsAnyConditional::TsConditionalType(conditional) => conditional.colon_token(),
        }
    }

    fn alternate(&self) -> SyntaxResult<ExpressionOrType> {
        match self {
            JsAnyConditional::JsConditionalExpression(conditional) => {
                conditional.alternate().map(ExpressionOrType::from)
            }
            JsAnyConditional::TsConditionalType(conditional) => {
                conditional.false_type().map(ExpressionOrType::from)
            }
        }
    }

    /// Returns `true` if the passed node is the `alternate` of this conditional expression.
    fn is_alternate(&self, node: &JsSyntaxNode) -> bool {
        let alternate = match self {
            JsAnyConditional::JsConditionalExpression(conditional) => {
                conditional.alternate().map(AstNode::into_syntax).ok()
            }
            JsAnyConditional::TsConditionalType(ts_conditional) => {
                ts_conditional.false_type().ok().map(AstNode::into_syntax)
            }
        };

        alternate.as_ref() == Some(node)
    }

    const fn is_conditional_expression(&self) -> bool {
        matches!(self, JsAnyConditional::JsConditionalExpression(_))
    }
}

fn is_jsx_conditional_chain(outer_most: &JsConditionalExpression) -> bool {
    fn recurse(expression: SyntaxResult<JsAnyExpression>) -> bool {
        use JsAnyExpression::*;

        match expression {
            Ok(JsConditionalExpression(conditional)) => is_jsx_conditional_chain(&conditional),
            Ok(JsxTagExpression(_)) => true,
            _ => false,
        }
    }

    recurse(outer_most.test())
        || recurse(outer_most.consequent())
        || recurse(outer_most.alternate())
}

fn format_jsx_chain_consequent(expression: &JsAnyExpression) -> FormatJsxChainExpression {
    FormatJsxChainExpression {
        expression,
        alternate: false,
    }
}

fn format_jsx_chain_alternate(alternate: &JsAnyExpression) -> FormatJsxChainExpression {
    FormatJsxChainExpression {
        expression: alternate,
        alternate: true,
    }
}

/// Wraps all expressions in parentheses if they break EXCEPT
/// * Nested conditionals in the alterante
/// * `null`
/// * `undefined`
struct FormatJsxChainExpression<'a> {
    expression: &'a JsAnyExpression,
    alternate: bool,
}

impl Format<JsFormatContext> for FormatJsxChainExpression<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        use JsAnyExpression::*;

        let no_wrap = match self.expression {
            JsIdentifierExpression(identifier) if identifier.name()?.is_undefined() => true,
            JsAnyLiteralExpression(
                rome_js_syntax::JsAnyLiteralExpression::JsNullLiteralExpression(_),
            ) => true,
            JsConditionalExpression(_) if self.alternate => true,
            _ => false,
        };

        let format_expression = format_with(|f| match self.expression {
            JsConditionalExpression(conditional) => {
                write!(
                    f,
                    [conditional
                        .format()
                        .with_options(ConditionalJsxChain::Chain)]
                )
            }
            expression => {
                write!(f, [expression.format()])
            }
        });

        if no_wrap {
            write!(f, [format_expression])
        } else {
            write!(
                f,
                [
                    if_group_breaks(&text("(")),
                    soft_block_indent(&format_expression),
                    if_group_breaks(&text(")"))
                ]
            )
        }
    }
}
