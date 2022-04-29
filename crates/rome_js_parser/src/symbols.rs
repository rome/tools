use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsComputedMemberExpression,
    JsComputedMemberExpressionFields, JsLiteralMemberName, JsLiteralMemberNameFields,
    JsReferenceIdentifier, JsReferenceIdentifierFields, JsStringLiteralExpression,
    JsStringLiteralExpressionFields, JsSyntaxKind, JsSyntaxNode, TextRange, TsGlobalDeclaration,
    TsGlobalDeclarationFields, TsThisParameter, TsThisParameterFields,
};
use rome_rowan::NodeOrToken;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Symbol {
    Declaration {
        name: String,
        range: TextRange,
    },
    Reference {
        name: String,
        range: TextRange,
        declared_at: Option<TextRange>,
    },
}

impl Symbol {
    pub fn name(&self) -> &str {
        match self {
            Symbol::Declaration { name, .. } => name,
            Symbol::Reference { name, .. } => name,
        }
    }

    pub fn range(&self) -> TextRange {
        match self {
            Symbol::Declaration { range, .. } => *range,
            Symbol::Reference { range, .. } => *range,
        }
    }
}

pub struct SymbolIterator(VecDeque<JsSyntaxNode>);

fn extract_symbol(node: JsSyntaxNode) -> Option<Symbol> {
    match node.kind() {
        JsSyntaxKind::JS_IDENTIFIER_BINDING
        | JsSyntaxKind::TS_IDENTIFIER_BINDING
        | JsSyntaxKind::JS_LITERAL_EXPORT_NAME => Some(Symbol::Declaration {
            name: node.text_trimmed().to_string(),
            range: node.text_range(),
        }),
        JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT
        | JsSyntaxKind::JS_SUPER_EXPRESSION
        | JsSyntaxKind::JS_THIS_EXPRESSION
        | JsSyntaxKind::JS_MODULE_SOURCE => Some(Symbol::Reference {
            name: node.text_trimmed().to_string(),
            range: node.text_range(),
            declared_at: None,
        }),
        // Some reference identifiers are not really references
        // - const on typescript const cast "10 as const"
        // - undefined
        JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
            let value_token = unsafe { JsReferenceIdentifier::new_unchecked(node) }
                .as_fields()
                .value_token
                .ok()?;

            match value_token.text_trimmed() {
                "const" | "undefined" => None,
                text_trimmed => Some(Symbol::Reference {
                    name: text_trimmed.to_string(),
                    range: value_token.text_range(),
                    declared_at: None,
                }),
            }
        }
        // JS_LITERAL_MEMBER_NAME to be a symbol:
        // - it cannot be a constructor
        // - it cannot be a string literal
        JsSyntaxKind::JS_LITERAL_MEMBER_NAME => {
            let is_inside_constructor = matches!(
                node.parent()?.kind(),
                JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER
                    | JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER
            );

            let value = unsafe { JsLiteralMemberName::new_unchecked(node) }
                .as_fields()
                .value
                .ok()?;
            let is_string_literal = matches!(value.kind(), JsSyntaxKind::JS_STRING_LITERAL);

            (!is_inside_constructor && !is_string_literal).then(|| Symbol::Declaration {
                name: value.text_trimmed().to_string(),
                range: value.text_range(),
            })
        }
        //
        // is JS_NAME under TS_NAMED_TUPLE_TYPE_ELEMENT a symbol?
        // example: type A = [ b: string ]; // <-- is b a symbol?
        JsSyntaxKind::JS_NAME => {
            let parent_kind = node.parent()?.kind();
            let parent_ok = matches!(
                parent_kind,
                JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
                    | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                    | JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE
                    | JsSyntaxKind::TS_QUALIFIED_MODULE_NAME
                    | JsSyntaxKind::TS_QUALIFIED_NAME
            );
            parent_ok.then(|| Symbol::Reference {
                name: node.text_trimmed().to_string(),
                range: node.text_range(),
                declared_at: None,
            })
        }
        JsSyntaxKind::TS_THIS_PARAMETER => {
            let this_token = unsafe { TsThisParameter::new_unchecked(node) }
                .as_fields()
                .this_token
                .ok()?;

            Some(Symbol::Declaration {
                name: this_token.text_trimmed().to_string(),
                range: this_token.text_range(),
            })
        }
        JsSyntaxKind::TS_GLOBAL_DECLARATION => {
            let global_token = unsafe { TsGlobalDeclaration::new_unchecked(node) }
                .as_fields()
                .global_token
                .ok()?;

            Some(Symbol::Declaration {
                name: global_token.text_trimmed().to_string(),
                range: global_token.text_range(),
            })
        }
        JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
            let value_token = unsafe { JsComputedMemberExpression::new_unchecked(node) }
                .as_fields()
                .member
                .ok()?
                .as_js_any_literal_expression()?
                .as_js_string_literal_expression()?
                .as_fields()
                .value_token
                .ok()?;

            Some(Symbol::Reference {
                name: value_token.text_trimmed().to_string(),
                range: value_token.text_range(),
                declared_at: None,
            })
        }
        JsSyntaxKind::TS_TYPE_PARAMETER_NAME => {
            let parent = node.parent()?;
            let great_parent = parent.parent()?;

            let is_in_type_parameter_list =
                matches!(great_parent.kind(), JsSyntaxKind::TS_TYPE_PARAMETER_LIST);
            let is_in_mapped_type = matches!(parent.kind(), JsSyntaxKind::TS_MAPPED_TYPE);

            (is_in_type_parameter_list || is_in_mapped_type).then(|| Symbol::Declaration {
                name: node.text_trimmed().to_string(),
                range: node.text_range(),
            })
        }
        _ => None,
    }
}

impl Iterator for SymbolIterator {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.0.pop_front() {
            for child in node.children() {
                self.0.push_back(child);
            }

            if let Some(s) = extract_symbol(node) {
                return Some(s);
            }
        }

        None
    }
}

pub fn symbols(root: JsSyntaxNode) -> SymbolIterator {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    SymbolIterator(queue)
}
