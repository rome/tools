use bitflags::bitflags;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionCategory {
    /// This action provides a fix to the diagnostic emitted by the same signal
    QuickFix,
    /// This action provides an optional refactor opportunity
    Refactor,
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
