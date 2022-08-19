use crate::prelude::*;
use rome_formatter::write;


use rome_js_syntax::{
    JsAnyExpression, JsAssignmentExpression, JsCallExpression, JsComputedMemberExpression,
    JsConditionalExpression, JsInitializerClause, JsNewExpression,
    JsReturnStatement, JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
    JsThrowStatement, JsUnaryExpression, JsYieldArgument, TsAsExpression, TsConditionalType,
    TsNonNullAssertionExpression, TsType,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxResult};

declare_node_union! {
    pub JsAnyConditional = JsConditionalExpression | TsConditionalType
}

impl Format<JsFormatContext> for JsAnyConditional {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let syntax = self.syntax();
        let consequent = self.consequent()?;
        let alternate = self.alternate()?;

        let layout = self.layout();

        let format_consequent_and_alternate = format_with(|f| {
            write!(
                f,
                [
                    soft_line_break_or_space(),
                    self.question_mark_token().format(),
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
                    self.colon_token().format(),
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
        });

        let should_extra_indent = self.should_extra_indent(&layout);

        let format_inner = format_with(|f| {
            write!(
                f,
                [FormatConditionalTest {
                    conditional: self,
                    layout: &layout,
                }]
            )?;

            // Indent the `consequent` and `alternate` **only if** this is the root conditional expression
            // OR this is the `test` of a conditional expression.
            if layout.is_root() || layout.is_nested_test() {
                write!(f, [indent(&format_tail_with_indent)])?;
            } else {
                // Don't indent for nested `alternate`s or `consequence`s
                write!(f, [format_tail_with_indent])?;
            }

            let break_closing_parentheses = self.is_parent_static_member_expression(&layout);

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
    // 	? consequent
    // 	: nestedAlternate +
    // 	  binary + // <- notice how the content is aligned to the `: `
    // 	? consequentOfnestedAlternate
    // 	: alternateOfNestedAlternate;
    // ```
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
    // 	? condition2
    // 		? consequent2 // <-- consequent and alternate gets indented
    // 		: alternate2
    // 	: alternate1;
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
    },
}

impl ConditionalLayout {
    const fn is_root(&self) -> bool {
        matches!(self, ConditionalLayout::Root { .. })
    }

    /// Returns the parent node, if any
    fn parent(&self) -> Option<&JsSyntaxNode> {
        match self {
            ConditionalLayout::NestedAlternate { parent }
            | ConditionalLayout::NestedTest { parent }
            | ConditionalLayout::NestedConsequent { parent } => Some(parent.syntax()),
            ConditionalLayout::Root { parent } => parent.as_ref(),
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
    fn layout(&self) -> ConditionalLayout {
        let resolved_parent = match self {
            JsAnyConditional::JsConditionalExpression(conditional) => conditional.syntax().parent(),
            JsAnyConditional::TsConditionalType(ty) => ty.syntax().parent(),
        };

        let parent = match resolved_parent {
            None => return ConditionalLayout::Root { parent: None },
            Some(parent) => parent,
        };

        if parent.kind() == self.syntax().kind() {
            let conditional_parent = JsAnyConditional::unwrap_cast(parent);

            if conditional_parent.is_test(self.syntax()) {
                ConditionalLayout::NestedTest {
                    parent: conditional_parent,
                }
            } else if conditional_parent.is_alternate(self.syntax()) {
                ConditionalLayout::NestedAlternate {
                    parent: conditional_parent,
                }
            } else {
                ConditionalLayout::NestedConsequent {
                    parent: conditional_parent,
                }
            }
        } else {
            ConditionalLayout::Root {
                parent: Some(parent),
            }
        }
    }

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
                        == Ok(self.syntax())
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
    fn should_extra_indent(&self, layout: &ConditionalLayout) -> bool {
        enum Ancestor {
            MemberChain(JsAnyExpression),
            Root(JsSyntaxNode),
        }

        let conditional = match self {
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

        // This tries to find the start of a member chain by iterating over all ancestors of the conditional
        // expression, while skipping parenthesized expression.
        // The iteration "breaks" as soon as a non-member-chain node is found.
        for ancestor in ancestors {
            let ancestor = match ancestor.kind() {
                JsSyntaxKind::JS_CALL_EXPRESSION => {
                    let call_expression = JsCallExpression::unwrap_cast(ancestor);

                    if call_expression.callee().as_ref() == Ok(&expression) {
                        Ancestor::MemberChain(call_expression.into())
                    } else {
                        Ancestor::Root(call_expression.into_syntax())
                    }
                }

                JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                    let member_expression = JsStaticMemberExpression::unwrap_cast(ancestor);

                    if member_expression.object().as_ref() == Ok(&expression) {
                        Ancestor::MemberChain(member_expression.into())
                    } else {
                        Ancestor::Root(member_expression.into_syntax())
                    }
                }
                JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                    let member_expression = JsComputedMemberExpression::unwrap_cast(ancestor);

                    if member_expression.object().as_ref() == Ok(&expression) {
                        Ancestor::MemberChain(member_expression.into())
                    } else {
                        Ancestor::Root(member_expression.into_syntax())
                    }
                }
                JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => {
                    let non_null_assertion = TsNonNullAssertionExpression::unwrap_cast(ancestor);

                    if non_null_assertion.expression().as_ref() == Ok(&expression) {
                        Ancestor::MemberChain(non_null_assertion.into())
                    } else {
                        Ancestor::Root(non_null_assertion.into_syntax())
                    }
                }
                JsSyntaxKind::JS_NEW_EXPRESSION => {
                    let new_expression = JsNewExpression::unwrap_cast(ancestor);

                    // Skip over new expressions
                    if new_expression.callee().as_ref() == Ok(&expression) {
                        parent = new_expression.syntax().parent();
                        expression = new_expression.into();
                        break;
                    }

                    Ancestor::Root(new_expression.into_syntax())
                }
                JsSyntaxKind::TS_AS_EXPRESSION => {
                    let as_expression = TsAsExpression::unwrap_cast(ancestor.clone());

                    if as_expression.expression().as_ref() == Ok(&expression) {
                        parent = as_expression.syntax().parent();
                        expression = as_expression.into();
                        break;
                    }

                    Ancestor::Root(as_expression.into_syntax())
                }
                _ => Ancestor::Root(ancestor),
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
    fn is_parent_static_member_expression(&self, layout: &ConditionalLayout) -> bool {
        if !self.is_conditional_expression() {
            return false;
        }

        match layout {
            ConditionalLayout::Root {
                parent: Some(parent),
            } => JsStaticMemberExpression::can_cast(parent.kind()),
            _ => false,
        }
    }
}
