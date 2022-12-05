use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsReferenceIdentifier, JsSyntaxKind, TextRange, TsObjectType, TsReferenceType,
};
use rome_rowan::{declare_node_union, AstNodeList, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow certain types.
    ///
    /// > Some built-in types have aliases, while some types are considered dangerous or harmful. It's often a good idea to ban certain types to help with consistency and safety.
    ///
    /// > This rule bans specific types and can suggest alternatives. Note that it doesn't ban the corresponding runtime objects from being used.
    ///
    /// Source: https://typescript-eslint.io/rules/ban-types
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let foo: String = "bar";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let bool = true as Boolean;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let invalidTuple: [string, Boolean] = ["foo", false];
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let foo: string = "bar";
    /// ```
    ///
    /// ```ts
    /// let tuple: [boolean, string] = [false, "foo"];
    /// ```
    ///
    /// ```
    pub(crate) NoBannedTypes {
        version: "10.0.0",
        name: "noBannedTypes",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) TsBannedType = TsReferenceType | TsObjectType
}
pub enum BannedType {
    BigInt,
    Boolean,
    Function,
    Number,
    Object,
    String,
    Symbol,
    /// {}
    EmptyObject,
}

impl BannedType {
    /// construct a [BannedType] from the textual name of a JavaScript type
    fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "BigInt" => Self::BigInt,
            "Boolean" => Self::Boolean,
            "Function" => Self::Function,
            "Number" => Self::Number,
            "Object" => Self::Object,
            "String" => Self::String,
            "Symbol" => Self::Symbol,
            "{}" => Self::EmptyObject,
            _ => return None,
        })
    }

    /// Convert a [BannedType] to a JS string literal
    fn as_str(&self) -> &'static str {
        match self {
            Self::BigInt => "BigInt",
            Self::Boolean => "Boolean",
            Self::Function => "Function",
            Self::Number => "Number",
            Self::Object => "Object",
            Self::String => "String",
            Self::Symbol => "Symbol",
            Self::EmptyObject => "{}",
        }
    }

    /// Retrieves a diagnostic message from a [BannedType]
    fn message(&self) -> &str {
        match *self {
			| Self::BigInt
			| Self::Boolean
			| Self::Number
			| Self::String
			| Self::Symbol => "Use lowercase primitives for consistency.",
			Self::Function =>
				"Prefer explicitly define the function shape. This type accepts any function-like value, which can be a common source of bugs.",
			Self::Object =>
				"Prefer explicitly define the object shape. This type means \"any non-nullish value\", which is slightly better than 'unknown', but it's still a broad type.",
			Self::EmptyObject => "Prefer explicitly define the object shape. '{}' means \"any non-nullish value\".",
		}
    }

    /// Converts a [BannedType] to a [JsSyntaxKind]
    fn as_js_syntax_kind(&self) -> Option<JsSyntaxKind> {
        Some(match *self {
            Self::BigInt => JsSyntaxKind::BIGINT_KW,
            Self::Boolean => JsSyntaxKind::BOOLEAN_KW,
            Self::Number => JsSyntaxKind::NUMBER_KW,
            Self::String => JsSyntaxKind::STRING_KW,
            Self::Symbol => JsSyntaxKind::SYMBOL_KW,
            _ => return None,
        })
    }

    /// Retrieves a [JsReferenceIdentifier] from a [BannedType] that will be used to
    /// replace it on the rule action
    fn fix_with(&self) -> Option<JsReferenceIdentifier> {
        Some(match *self {
            Self::BigInt | Self::Boolean | Self::Number | Self::String | Self::Symbol => {
                make::js_reference_identifier(make::token(Self::as_js_syntax_kind(self)?))
            }
            _ => return None,
        })
    }
}

pub struct RuleState {
    /// Reference to the enum item containing the banned type.
    /// Used for both diagnostic and action.
    banned_type: BannedType,
    /// Text range used to diagnostic the banned type.
    banned_type_range: TextRange,
    /// Reference to the node to be replaced in the action.
    /// This is optional because we don't replace empty objects references.
    reference_identifier: Option<JsReferenceIdentifier>,
}

impl Rule for NoBannedTypes {
    type Query = Ast<TsBannedType>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();

        match query {
            TsBannedType::TsObjectType(ts_object_type) => {
                if ts_object_type.members().is_empty() {
                    let range = TextRange::new(
                        ts_object_type
                            .l_curly_token()
                            .ok()?
                            .text_trimmed_range()
                            .start(),
                        ts_object_type
                            .r_curly_token()
                            .ok()?
                            .text_trimmed_range()
                            .end(),
                    );

                    return Some(RuleState {
                        banned_type: BannedType::EmptyObject,
                        banned_type_range: range,
                        reference_identifier: None,
                    });
                }
            }
            TsBannedType::TsReferenceType(ts_reference_type) => {
                let ts_any_name = ts_reference_type.name().ok()?;
                let reference_identifier = ts_any_name.as_js_reference_identifier()?;
                let identifier_token = reference_identifier.value_token().ok()?;

                if let Some(banned_type) = BannedType::from_str(identifier_token.text_trimmed()) {
                    return Some(RuleState {
                        banned_type,
                        banned_type_range: identifier_token.text_trimmed_range(),
                        reference_identifier: Some(reference_identifier.clone()),
                    });
                }
            }
        }

        None
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        RuleState {
            banned_type,
            banned_type_range,
            ..
        }: &Self::State,
    ) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            banned_type_range,
            markup! {"Don't use '"{banned_type.as_str()}"' as a type."}.to_owned(),
        )
        .note(markup! { {banned_type.message()} }.to_owned());

        Some(diagnostic)
    }

    fn action(
        ctx: &RuleContext<Self>,
        RuleState {
            banned_type,
            reference_identifier,
            ..
        }: &Self::State,
    ) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let suggested_type = banned_type.as_js_syntax_kind()?.to_string()?;

        mutation.replace_node(reference_identifier.clone()?, banned_type.fix_with()?);

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Use '"{suggested_type}"' instead" }.to_owned(),
            mutation,
        })
    }
}
