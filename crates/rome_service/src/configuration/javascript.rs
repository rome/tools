use indexmap::IndexSet;
use rome_js_formatter::context::QuoteStyle;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JavascriptConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<JavascriptFormatter>,

    /// A list of global bindings that should be ignored by the analyzers
    ///
    /// If defined here, they should not emit diagnostics.
    #[serde(
        skip_serializing_if = "IndexSet::is_empty",
        deserialize_with = "deserialize_globals",
        serialize_with = "serialize_globals"
    )]
    pub globals: IndexSet<String>,
}

pub(crate) fn deserialize_globals<'de, D>(deserializer: D) -> Result<IndexSet<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_seq(IndexVisitor::new())
}

struct IndexVisitor {
    marker: PhantomData<fn() -> IndexSet<String>>,
}

impl IndexVisitor {
    fn new() -> Self {
        IndexVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for IndexVisitor {
    type Value = IndexSet<String>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expecting a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut index_set = IndexSet::with_capacity(seq.size_hint().unwrap_or(0));

        while let Some(value) = seq.next_element()? {
            index_set.insert(value);
        }

        Ok(index_set)
    }
}

pub(crate) fn serialize_globals<S>(globals: &IndexSet<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    let mut sequence = s.serialize_seq(Some(globals.len()))?;
    let iter = globals.into_iter();
    for global in iter {
        sequence.serialize_element(global)?;
    }
    sequence.end()
}

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JavascriptFormatter {
    /// The style for quotes. Defaults to double.
    #[serde(with = "PlainQuoteStyle")]
    pub quote_style: QuoteStyle,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", remote = "QuoteStyle")]
pub enum PlainQuoteStyle {
    #[default]
    Double,
    Single,
}
