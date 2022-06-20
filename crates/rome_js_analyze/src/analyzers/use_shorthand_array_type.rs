use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyStatement, JsForStatement, JsForStatementFields, TriviaPieceKind, TsReferenceType, TsType,
    TsTypeAnnotation, TsTypeArguments, T,
};
use rome_rowan::{AstNode, AstNodeExt, AstSeparatedList};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce the use of `while` loops instead of `for` loops when the
    /// initializer and update expressions are not needed
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (; x.running;) {
    ///     x.step();
    /// }
    /// ```
    pub(crate) UseShorthandArrayType = "useShorthandArrayType"
}

impl Rule for UseShorthandArrayType {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = TsReferenceType;
    type State = TsType;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        if !is_array_reference(&node).unwrap_or(false) || node.type_arguments().is_none() {
            return None;
        }
        // SAFETY: We have checked the `node.type_arguments` is `Some` above, if it `None`, it would be early returned.
        let type_arguments = node.type_arguments().unwrap();

        convert_to_array_type(type_arguments)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        // SAFETY: These tokede.r_paren_token().unwrap().text_trimmed_range();

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Use shorthand T[] syntax instead of Array<T> syntax."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let root = ctx.root();
        let node = ctx.query();
        let root = root.replace_node(TsType::TsReferenceType(node.clone()), state.clone())?;
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a [] to replace" }.to_owned(),
            root,
        })
    }
}

fn is_array_reference(ty: &TsReferenceType) -> Option<bool> {
    let name = ty.name().ok()?;
    name.as_js_reference_identifier().and_then(|identifier| {
        let name = identifier.value_token().ok()?;
        Some(name.text_trimmed() == "Array")
    })
}

fn convert_to_array_type(type_arguments: TsTypeArguments) -> Option<TsType> {
    if type_arguments.ts_type_argument_list().len() > 0 {
        let mut array_types = Vec::new();

        for param in type_arguments.ts_type_argument_list().iter() {
            let param = param.ok()?;
            let element_type = match param {
                TsType::TsUnionType(_) => continue,
                TsType::TsTypeOperatorType(_) => continue,
                TsType::TsReferenceType(ty)
                    if is_array_reference(&ty).unwrap_or(false) && ty.type_arguments().is_some() =>
                {
                    // SAFETY: We have checked the `ty.type_arguments` is `Some` in match guard
                    convert_to_array_type(ty.type_arguments().unwrap())
                }
                _ => Some(param),
            };
            println!("{:?}", element_type);
            if let Some(element_type) = element_type {
                array_types.push(TsType::TsArrayType(make::ts_array_type(
                    element_type,
                    make::token(T!['[']),
                    make::token(T![']']),
                )));
            }
            // let param_type = param.ts_type().ok()?;
            // array_types.push(make::ts_type_array(param_type.clone()));
        }
        println!(
            "{:?}------------------------------------------",
            array_types
        );
        match array_types.len() {
            0 => {}
            1 => {
                // SAFETY: We know that `length` of `array_types` is 1, so unwrap the first element should be safe.
                let first_type = array_types.into_iter().next().unwrap();
                return Some(first_type);
            }
            length => {
                let ts_union_type_builder = make::ts_union_type(make::ts_union_type_variant_list(
                    array_types.into_iter().enumerate().map(|(i, item)| {
                        (
                            item,
                            (i != length - 1).then(|| {
                                let separator = make::token(T![|])
                                    .with_leading_trivia(std::iter::once((
                                        TriviaPieceKind::Whitespace,
                                        " ",
                                    )))
                                    .with_trailing_trivia(std::iter::once((
                                        TriviaPieceKind::Whitespace,
                                        " ",
                                    )));
                                separator
                            }),
                        )
                    }),
                ));
                // return Some()e
                return Some(TsType::TsUnionType(ts_union_type_builder.build()));
            }
        }
    }
    println!("fuck: {}", type_arguments.ts_type_argument_list().len());
    None
}
