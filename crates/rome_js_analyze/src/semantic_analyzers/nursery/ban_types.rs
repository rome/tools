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
    /// Disallow certain types
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
    pub(crate) BanTypes {
        version: "10.0.0",
        name: "banTypes",
        recommended: false,
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
    EmptyObject, // {}
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
    fn message(&self) -> String {
        match *self {
			Self::BigInt => String::from("Use bigint instead"),
			Self::Boolean => String::from("Use boolean instead"),
			Self::Function => [
				"The `Function` type accepts any function-like value.",
				"It provides no type safety when calling the function, which can be a common source of bugs.",
				"It also accepts things like class declarations, which will throw at runtime as they will not be called with `new`.",
				"If you are expecting the function to accept certain arguments, you should explicitly define the function shape",
			].join("\n"),
			Self::Number => String::from("Use number instead"),
			Self::Object => [
				"The `Object` type actually means \"any non-nullish value\", so it is marginally better than `unknown`.",
				"- If you want a type meaning \"any object\", you probably want `Record<string, unknown>` instead.",
				"- If you want a type meaning \"any value\", you probably want `unknown` instead",
			].join("\n"),
			Self::String => String::from("Use string instead"),
			Self::Symbol => String::from("Use symbol instead"),
			Self::EmptyObject => [
				"`{}` actually means \"any non-nullish value\".",
				"- If you want a type meaning \"any object\", you probably want `Record<string, unknown>` instead.",
				"- If you want a type meaning \"any value\", you probably want `unknown` instead.",
				"- If you want a type meaning \"empty object\", you probably want `Record<string, never>` instead",
			].join("\n"),
		}
    }

    /// Retrieves a [JsSyntaxKind] from a [BannedType] that will be used to
    /// replace it on the rule action
    fn fix_with(&self) -> Option<JsSyntaxKind> {
        Some(match *self {
            Self::BigInt => JsSyntaxKind::BIGINT_KW,
            Self::Boolean => JsSyntaxKind::BOOLEAN_KW,
            Self::Number => JsSyntaxKind::NUMBER_KW,
            Self::String => JsSyntaxKind::STRING_KW,
            Self::Symbol => JsSyntaxKind::SYMBOL_KW,
            _ => return None,
        })
    }
}

impl Rule for BanTypes {
    type Query = Ast<TsBannedType>;
    type State = (BannedType, TextRange, Option<JsReferenceIdentifier>);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();

        match query {
            TsBannedType::TsObjectType(ts_object_type) => {
                if ts_object_type.members().is_empty() {
                    let range = TextRange::new(
                        ts_object_type.l_curly_token().ok()?.text_range().start(),
                        ts_object_type.r_curly_token().ok()?.text_range().end(),
                    );

                    return Some((BannedType::EmptyObject, range, None));
                }
            }
            TsBannedType::TsReferenceType(ts_reference_type) => {
                let ts_any_name = ts_reference_type.name().ok()?;
                let reference_identifier = ts_any_name.as_js_reference_identifier()?;
                let identifier_token = reference_identifier.value_token().ok()?;

                if let Some(banned_type) = BannedType::from_str(identifier_token.text_trimmed()) {
                    return Some((
                        banned_type,
                        identifier_token.text_trimmed_range(),
                        Some(reference_identifier.clone()),
                    ));
                }
            }
        }

        None
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        (banned_type, text_range, ..): &Self::State,
    ) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            text_range,
            markup! {"Don't use "<Emphasis>{banned_type.as_str()}</Emphasis>" as a type. "<Emphasis>{banned_type.message()}</Emphasis>"."}.to_owned(),
        );

        Some(diagnostic)
    }

    fn action(
        ctx: &RuleContext<Self>,
        (banned_type, .., reference_identifier): &Self::State,
    ) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let new_token = banned_type.fix_with()?;
        let refs = reference_identifier.as_ref()?;

        mutation.replace_node(
            refs.clone(),
            make::js_reference_identifier(make::token(new_token)),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Fix this problem" }.to_owned(),
            mutation,
        })
    }
}
