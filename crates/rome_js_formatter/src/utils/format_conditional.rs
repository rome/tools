use crate::prelude::*;
use rome_formatter::{format_args, write, CstFormatContext, FormatContext};
use rome_js_syntax::{
    JsAnyExpression, JsCallExpression, JsConditionalExpression, JsStaticMemberExpression,
    JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, TsConditionalType, TsType,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxNode, SyntaxResult};
use std::cmp::max;

declare_node_union! {
    pub JsAnyConditional = JsConditionalExpression | TsConditionalType
}

impl Format<JsFormatContext> for JsAnyConditional {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let syntax = self.syntax();
        let consequent = self.consequent()?;
        let alternate = self.alternate()?;

        let parent = syntax.parent();
        let parent_conditional = parent
            .as_ref()
            .and_then(|parent| JsAnyConditional::cast(parent.clone()));

        let is_nested = parent_conditional.is_some();
        let is_test_of_parent = parent_conditional
            .as_ref()
            .map_or(false, |conditional| conditional.is_test(syntax));
        let is_alternate_of_parent = parent_conditional
            .as_ref()
            .map_or(false, |conditional| conditional.is_alternate(syntax));

        let format_tail_raw = format_with(|f| {
            write!(
                f,
                [
                    soft_line_break_or_space(),
                    self.question_mark_token().format(),
                    space()
                ]
            )?;

            if consequent.syntax().kind() == syntax.kind() {
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

            if alternate.syntax().kind() == syntax.kind() {
                write!(f, [alternate])
            } else {
                write!(f, [align(2, &alternate),])
            }
        });

        let format_tail = format_with(|f| {
            if !is_nested || is_test_of_parent || is_alternate_of_parent {
                write!(f, [format_tail_raw])
            } else if f.context().indent_style().is_tab() {
                // TODO add dedent & indent Probably to remove the outer align and convert it to an indent?
                write!(f, [format_tail_raw])
            } else {
                let align_count = u8::from(f.context().tab_width()).max(2u8) - 2u8;

                if align_count == 0 {
                    write!(f, [format_tail_raw])
                } else {
                    write!(f, [align(align_count, &format_tail_raw)])
                }
            }
        });

        let force_no_indent = is_nested && !is_test_of_parent;
        let break_closing_parentheses = !is_nested
            && parent.map_or(false, |parent| {
                JsStaticMemberExpression::can_cast(parent.kind())
            });

        let should_extra_indent = self.should_extra_indent();

        let format_test = format_with(|f| {
            let format_test_plain = format_with(|f| match self {
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

            if is_alternate_of_parent {
                write!(f, [align(2, &format_test_plain)])
            } else {
                write!(f, [format_test_plain])
            }
        });

        let format_inner = format_with(|f| {
            write!(f, [format_test])?;

            if force_no_indent {
                write!(f, [format_tail])?;
            } else {
                write!(f, [indent(&format_tail)])?;
            }

            if self.is_conditional_expression() && break_closing_parentheses && !should_extra_indent
            {
                write!(f, [soft_line_break()])?;
            }

            Ok(())
        });

        // TODO should break

        if !is_nested {
            write!(f, [group(&format_inner)])
        } else {
            write!(f, [format_inner])
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

enum ConditionalLayout {
    Nested,
    Default,
}

impl JsAnyConditional {
    fn test(&self) -> SyntaxResult<ExpressionOrType> {
        match self {
            JsAnyConditional::JsConditionalExpression(conditional) => {
                conditional.test().map(ExpressionOrType::from)
            }
            JsAnyConditional::TsConditionalType(conditional) => {
                conditional.check_type().map(ExpressionOrType::from)
            }
        }
    }

    /// Returns `true` if `node` is the `test` of this conditional.
    fn is_test(&self, node: &JsSyntaxNode) -> bool {
        match self {
            JsAnyConditional::JsConditionalExpression(conditional) => {
                conditional.test().map(AstNode::into_syntax).as_ref() == Ok(node)
            }
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

    fn is_alternate(&self, node: &JsSyntaxNode) -> bool {
        self.alternate().map(AstNode::into_syntax).as_ref() == Ok(node)
    }

    fn try_from(expression_or_type: ExpressionOrType) -> Option<Self> {
        match expression_or_type {
            ExpressionOrType::JsAnyExpression(JsAnyExpression::JsConditionalExpression(
                expression,
            )) => Some(JsAnyConditional::from(expression)),
            ExpressionOrType::TsType(TsType::TsConditionalType(ty)) => {
                Some(JsAnyConditional::from(ty))
            }
            _ => None,
        }
    }

    const fn is_conditional_expression(&self) -> bool {
        matches!(self, JsAnyConditional::JsConditionalExpression(_))
    }

    fn should_extra_indent(&self) -> bool {
        if !self.is_conditional_expression() {
            return false;
        }

        // let mut chain =
        //     std::iter::successors(self.syntax().parent(), |parent| match ancestor.kind() {
        //         JsSyntaxKind::JS_CALL_EXPRESSION => {
        //             let call_expression = JsCallExpression::unwrap_cast(ancestor);
        //             let callee = call_expression.callee()?.into_syntax();
        //
        //             if &callee == &child {
        //                 return Some(callee);
        //             }
        //
        //             None
        //         }
        //         JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
        //             let member_expression = JsStaticMemberExpression::unwrap_cast(ancestor);
        //             let object = member_expression.object()?.into_syntax();
        //
        //             if &object == &child {
        //                 return Some(object);
        //             }
        //
        //             None
        //         }
        //         JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => {
        //             let non_null_assertion = JsStaticMemberExpression::unwrap_cast(ancestor);
        //             let inner = non_null_assertion.object()?.into_syntax();
        //
        //             if &inner == &child {
        //                 return Some(inner);
        //             }
        //
        //             None
        //         }
        //         _ => None,
        //     });
        //
        // let root = chain.last();

        // let mut child = self.syntax().clone();
        //
        // for ancestor in self.syntax().ancestors().skip(1) {

        //     }
        // }
        //
        // if &child == self.syntax() {
        //     false
        // } else {
        //     parent
        // }

        // TODO
        false

        //   return parent[ancestorNameMap.get(parent.type)] === child;

        //   let parent;
        //   let child = node;
        //   for (let ancestorCount = 0; !parent; ancestorCount++) {
        //     const node = path.getParentNode(ancestorCount);
        //
        //     if (
        //       (isCallExpression(node) && node.callee === child) ||
        //       (isMemberExpression(node) && node.object === child) ||
        //       (node.type === "TSNonNullExpression" && node.expression === child)
        //     ) {
        //       child = node;
        //       continue;
        //     }
        //
        //     // Reached chain root
        //
        //     if (
        //       (node.type === "NewExpression" && node.callee === child) ||
        //       (node.type === "TSAsExpression" && node.expression === child)
        //     ) {
        //       parent = path.getParentNode(ancestorCount + 1);
        //       child = node;
        //     } else {
        //       parent = node;
        //     }
        //   }
        //
        //   // Do not add indent to direct `ConditionalExpression`
        //   if (child === node) {
        //     return false;
        //   }
        //
        //   return parent[ancestorNameMap.get(parent.type)] === child;
        // }
    }
}

// const node = path.getValue();
//   const isConditionalExpression = node.type === "ConditionalExpression";
//   const consequentNodePropertyName = isConditionalExpression
//     ? "consequent"
//     : "trueType";
//   const alternateNodePropertyName = isConditionalExpression
//     ? "alternate"
//     : "falseType";
//   const testNodePropertyNames = isConditionalExpression
//     ? ["test"]
//     : ["checkType", "extendsType"];
//   const consequentNode = node[consequentNodePropertyName];
//   const alternateNode = node[alternateNodePropertyName];
//   const parts = [];
//
//   // We print a ConditionalExpression in either "JSX mode" or "normal mode".
//   // See `tests/format/jsx/conditional-expression.js` for more info.
//   let jsxMode = false;
//   const parent = path.getParentNode();
//   const isParentTest =
//     parent.type === node.type &&
//     testNodePropertyNames.some((prop) => parent[prop] === node);
//   let forceNoIndent = parent.type === node.type && !isParentTest;
//
//   // Find the outermost non-ConditionalExpression parent, and the outermost
//   // ConditionalExpression parent. We'll use these to determine if we should
//   // print in JSX mode.
//   let currentParent;
//   let previousParent;
//   let i = 0;
//   do {
//     previousParent = currentParent || node;
//     currentParent = path.getParentNode(i);
//     i++;
//   } while (
//     currentParent &&
//     currentParent.type === node.type &&
//     testNodePropertyNames.every(
//       (prop) => currentParent[prop] !== previousParent
//     )
//   );
//   const firstNonConditionalParent = currentParent || parent;
//   const lastConditionalParent = previousParent;
//
//   if (
//     isConditionalExpression &&
//     (isJsxNode(node[testNodePropertyNames[0]]) ||
//       isJsxNode(consequentNode) ||
//       isJsxNode(alternateNode) ||
//       conditionalExpressionChainContainsJsx(lastConditionalParent))
//   ) {
//     jsxMode = true;
//     forceNoIndent = true;
//
//     // Even though they don't need parens, we wrap (almost) everything in
//     // parens when using ?: within JSX, because the parens are analogous to
//     // curly braces in an if statement.
//     const wrap = (doc) => [
//       ifBreak("("),
//       indent([softline, doc]),
//       softline,
//       ifBreak(")"),
//     ];
//
//     // The only things we don't wrap are:
//     // * Nested conditional expressions in alternates
//     // * null
//     // * undefined
//     const isNil = (node) =>
//       node.type === "NullLiteral" ||
//       (node.type === "Literal" && node.value === null) ||
//       (node.type === "Identifier" && node.name === "undefined");
//
//     parts.push(
//       " ? ",
//       isNil(consequentNode)
//         ? print(consequentNodePropertyName)
//         : wrap(print(consequentNodePropertyName)),
//       " : ",
//       alternateNode.type === node.type || isNil(alternateNode)
//         ? print(alternateNodePropertyName)
//         : wrap(print(alternateNodePropertyName))
//     );
//   } else {
//     // normal mode
//     const part = [
//       line,
//       "? ",
//       consequentNode.type === node.type ? ifBreak("", "(") : "",
//       align(2, print(consequentNodePropertyName)),
//       consequentNode.type === node.type ? ifBreak("", ")") : "",
//       line,
//       ": ",
//       alternateNode.type === node.type
//         ? print(alternateNodePropertyName)
//         : align(2, print(alternateNodePropertyName)),
//     ];
//     parts.push(
//       parent.type !== node.type ||
//         parent[alternateNodePropertyName] === node ||
//         isParentTest
//         ? part
//         : options.useTabs
//         ? dedent(indent(part))
//         : align(Math.max(0, options.tabWidth - 2), part)
//     );
//   }
//
//   // We want a whole chain of ConditionalExpressions to all
//   // break if any of them break. That means we should only group around the
//   // outer-most ConditionalExpression.
//   const comments = [
//     ...testNodePropertyNames.map((propertyName) =>
//       getComments(node[propertyName])
//     ),
//     getComments(consequentNode),
//     getComments(alternateNode),
//   ].flat();
//   const shouldBreak = comments.some(
//     (comment) =>
//       isBlockComment(comment) &&
//       hasNewlineInRange(
//         options.originalText,
//         locStart(comment),
//         locEnd(comment)
//       )
//   );
//   const maybeGroup = (doc) =>
//     parent === firstNonConditionalParent
//       ? group(doc, { shouldBreak })
//       : shouldBreak
//       ? [doc, breakParent]
//       : doc;
//
//   // Break the closing paren to keep the chain right after it:
//   // (a
//   //   ? b
//   //   : c
//   // ).call()
//   const breakClosingParen =
//     !jsxMode &&
//     (isMemberExpression(parent) ||
//       (parent.type === "NGPipeExpression" && parent.left === node)) &&
//     !parent.computed;
//
//   const shouldExtraIndent = shouldExtraIndentForConditionalExpression(path);
//
//   const result = maybeGroup([
//     printTernaryTest(path, options, print),
//     forceNoIndent ? parts : indent(parts),
//     isConditionalExpression && breakClosingParen && !shouldExtraIndent
//       ? softline
//       : "",
//   ]);
//
//   return isParentTest || shouldExtraIndent
//     ? group([indent([softline, result]), softline])
//     : result;
