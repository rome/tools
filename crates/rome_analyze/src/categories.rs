use std::borrow::Cow;

use bitflags::bitflags;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum RuleCategory {
    /// This rule checks the syntax according to the language specification
    /// and emits error diagnostics accordingly
    Syntax,
    /// This rule performs static analysis of the source code to detect
    /// invalid or error-prone patterns, and emits diagnostics along with
    /// proposed fixes
    Lint,
    /// This rule detects refactoring opportunities and emits code action
    /// signals
    Action,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum ActionCategory {
    /// This action provides a fix to the diagnostic emitted by the same signal
    QuickFix,
    /// This action provides an optional refactor opportunity
    Refactor(RefactorKind),
    Source(SourceActionKind),
    Other(Cow<'static, str>),
}

impl ActionCategory {
    pub fn matches(&self, filter: &str) -> bool {
        self.to_str().starts_with(filter)
    }

    pub fn to_str(&self) -> Cow<'static, str> {
        match self {
            ActionCategory::QuickFix => Cow::Borrowed("quickfix.rome"),

            ActionCategory::Refactor(RefactorKind::None) => Cow::Borrowed("refactor.rome"),
            ActionCategory::Refactor(RefactorKind::Extract) => {
                Cow::Borrowed("refactor.extract.rome")
            }
            ActionCategory::Refactor(RefactorKind::Inline) => Cow::Borrowed("refactor.inline.rome"),
            ActionCategory::Refactor(RefactorKind::Rewrite) => {
                Cow::Borrowed("refactor.rewrite.rome")
            }
            ActionCategory::Refactor(RefactorKind::Other(tag)) => {
                Cow::Owned(format!("refactor.{tag}.rome"))
            }

            ActionCategory::Source(SourceActionKind::None) => Cow::Borrowed("source.rome"),
            ActionCategory::Source(SourceActionKind::OrganizeImports) => {
                Cow::Borrowed("source.organizeImports.rome")
            }
            ActionCategory::Source(SourceActionKind::Other(tag)) => {
                Cow::Owned(format!("source.{tag}.rome"))
            }

            ActionCategory::Other(tag) => Cow::Owned(format!("{tag}.rome")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum RefactorKind {
    None,
    Extract,
    Inline,
    Rewrite,
    Other(Cow<'static, str>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum SourceActionKind {
    None,
    OrganizeImports,
    Other(Cow<'static, str>),
}

bitflags! {
    pub struct RuleCategories: u8 {
        const SYNTAX = 1 << RuleCategory::Syntax as u8;
        const LINT = 1 << RuleCategory::Lint as u8;
        const ACTION = 1 << RuleCategory::Action as u8;
    }
}

impl Default for RuleCategories {
    fn default() -> Self {
        Self::all()
    }
}

impl RuleCategories {
    pub fn is_syntax(&self) -> bool {
        *self == RuleCategories::SYNTAX
    }
}

impl From<RuleCategory> for RuleCategories {
    fn from(input: RuleCategory) -> Self {
        match input {
            RuleCategory::Syntax => RuleCategories::SYNTAX,
            RuleCategory::Lint => RuleCategories::LINT,
            RuleCategory::Action => RuleCategories::ACTION,
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RuleCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut flags = Vec::new();

        if self.contains(Self::SYNTAX) {
            flags.push(RuleCategory::Syntax);
        }

        if self.contains(Self::LINT) {
            flags.push(RuleCategory::Lint);
        }

        if self.contains(Self::ACTION) {
            flags.push(RuleCategory::Action);
        }

        serializer.collect_seq(flags)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RuleCategories {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, SeqAccess};
        use std::fmt::{self, Formatter};

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = RuleCategories;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "RuleCategories")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result = RuleCategories::empty();

                while let Some(item) = seq.next_element::<RuleCategory>()? {
                    result |= RuleCategories::from(item);
                }

                Ok(result)
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

#[cfg(feature = "serde")]
impl schemars::JsonSchema for RuleCategories {
    fn schema_name() -> String {
        String::from("RuleCategories")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <Vec<RuleCategory>>::json_schema(gen)
    }
}
