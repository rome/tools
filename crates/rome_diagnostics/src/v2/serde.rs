use std::{fmt::Write as _, io};

use rome_console::{fmt, markup, MarkupBuf};
use rome_text_edit::TextRange;
use serde::{
    de::{self, SeqAccess},
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::{
    diagnostic::DiagnosticTag, error::internal::AsDiagnostic, Backtrace, Category, DiagnosticTags,
    IntoAdvices, LogCategory, Path, Severity, SourceCode, Visitor,
};

/// Serializable representation for a [Diagnostic]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Diagnostic {
    category: Option<String>,
    severity: Severity,
    description: String,
    message: MarkupBuf,
    advices: Advices,
    verbose_advices: Advices,
    location: Option<Location>,
    tags: DiagnosticTags,
    source: Option<Box<Self>>,
}

impl Diagnostic {
    pub fn new<D: AsDiagnostic>(diag: D) -> Self {
        Self::new_impl(diag.as_diagnostic())
    }

    fn new_impl<D: super::Diagnostic + ?Sized>(diag: &D) -> Self {
        let category = diag.category().map(|category| category.name().to_string());

        let severity = diag.severity();

        let mut description = String::new();
        write!(description, "{}", PrintDescription(diag)).unwrap();

        let mut message = MarkupBuf::default();
        let mut fmt = fmt::Formatter::new(&mut message);
        // SAFETY: Writing to a MarkupBuf should never fail
        diag.message(&mut fmt).unwrap();

        let mut advices = Advices::new();
        // SAFETY: The Advices visitor never returns an error
        diag.advices(&mut advices).unwrap();

        let mut verbose_advices = Advices::new();
        // SAFETY: The Advices visitor never returns an error
        diag.verbose_advices(&mut verbose_advices).unwrap();

        let location = diag.location().map(Location::from);

        let tags = diag.tags();

        let source = diag.source().map(Self::new_impl).map(Box::new);

        Self {
            category,
            severity,
            description,
            message,
            advices,
            verbose_advices,
            location,
            tags,
            source,
        }
    }
}

impl super::Diagnostic for Diagnostic {
    fn category(&self) -> Option<&Category> {
        self.category.as_deref().and_then(|name| {
            let category: Option<&Category> = name.parse().ok();
            debug_assert!(
                category.is_some(),
                "diagnostic category {name:?} does not exist in the static registry"
            );
            category
        })
    }

    fn severity(&self) -> Severity {
        self.severity
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&self.description)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        fmt.write_markup(markup! { {self.message} })
    }

    fn advices(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        self.advices.visit(visitor)
    }

    fn verbose_advices(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        self.verbose_advices.visit(visitor)
    }

    fn location(&self) -> Option<super::Location<'_>> {
        self.location.as_ref().and_then(|location| {
            super::Location::builder()
                .path(&location.path)
                .span(&location.span)
                .source_code(&location.source_code)
                .build()
        })
    }

    fn tags(&self) -> DiagnosticTags {
        self.tags
    }

    fn source(&self) -> Option<&dyn super::Diagnostic> {
        self.source
            .as_deref()
            .map(|source| source as &dyn super::Diagnostic)
    }
}

/// Wrapper type implementing [std::fmt::Display] for types implementing [Diagnostic],
/// prints the description of the diagnostic as a string
struct PrintDescription<'fmt, D: ?Sized>(pub &'fmt D);

impl<'fmt, D: super::Diagnostic + ?Sized> std::fmt::Display for PrintDescription<'fmt, D> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.description(fmt).map_err(|_| std::fmt::Error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Location {
    path: Path<String>,
    span: Option<TextRange>,
    source_code: Option<String>,
}

impl From<super::Location<'_>> for Location {
    fn from(loc: super::Location<'_>) -> Self {
        Self {
            path: loc.path.to_owned(),
            span: loc.span,
            source_code: loc
                .source_code
                .map(|source_code| source_code.text.to_string()),
        }
    }
}

/// Implementation of [Visitor] collecting serializable [Advice] into a vector
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Advices {
    advices: Vec<Advice>,
}

impl Advices {
    fn new() -> Self {
        Self {
            advices: Vec::new(),
        }
    }
}

impl Visitor for Advices {
    fn visit_log(&mut self, category: LogCategory, text: &dyn fmt::Display) -> io::Result<()> {
        self.advices
            .push(Advice::Log(category, markup!({ text }).to_owned()));
        Ok(())
    }

    fn visit_list(&mut self, list: &[&dyn fmt::Display]) -> io::Result<()> {
        self.advices.push(Advice::List(
            list.iter()
                .map(|item| markup!({ item }).to_owned())
                .collect(),
        ));
        Ok(())
    }

    fn visit_frame(&mut self, location: super::Location<'_>) -> io::Result<()> {
        self.advices.push(Advice::Frame(location.into()));
        Ok(())
    }

    fn visit_diff(&mut self, prev: &str, next: &str) -> io::Result<()> {
        self.advices.push(Advice::Diff(prev.into(), next.into()));
        Ok(())
    }

    fn visit_backtrace(
        &mut self,
        title: &dyn fmt::Display,
        backtrace: &Backtrace,
    ) -> io::Result<()> {
        self.advices.push(Advice::Backtrace(
            markup!({ title }).to_owned(),
            backtrace.clone(),
        ));
        Ok(())
    }

    fn visit_command(&mut self, command: &str) -> io::Result<()> {
        self.advices.push(Advice::Command(command.into()));
        Ok(())
    }

    fn visit_group(
        &mut self,
        title: &dyn fmt::Display,
        advice: &dyn IntoAdvices,
    ) -> io::Result<()> {
        let mut advices = Advices::new();
        advice.visit(&mut advices)?;

        self.advices
            .push(Advice::Group(markup!({ title }).to_owned(), advices));
        Ok(())
    }
}

impl IntoAdvices for Advices {
    fn visit(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        for advice in &self.advices {
            advice.visit(visitor)?;
        }

        Ok(())
    }
}

/// Serializable representation of a [Diagnostic] advice
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(Eq, PartialEq))]
enum Advice {
    Log(LogCategory, MarkupBuf),
    List(Vec<MarkupBuf>),
    Frame(Location),
    Diff(String, String),
    Backtrace(MarkupBuf, Backtrace),
    Command(String),
    Group(MarkupBuf, Advices),
}

impl IntoAdvices for Advice {
    fn visit(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        match self {
            Advice::Log(category, text) => visitor.visit_log(*category, text),
            Advice::List(list) => {
                let as_display: Vec<&dyn fmt::Display> =
                    list.iter().map(|item| item as &dyn fmt::Display).collect();
                visitor.visit_list(&as_display)
            }
            Advice::Frame(location) => visitor.visit_frame(super::Location {
                path: location.path.as_deref(),
                span: location.span,
                source_code: location.source_code.as_deref().map(|text| SourceCode {
                    text,
                    line_starts: None,
                }),
            }),
            Advice::Diff(prev, next) => visitor.visit_diff(prev, next),
            Advice::Backtrace(title, backtrace) => visitor.visit_backtrace(title, backtrace),
            Advice::Command(command) => visitor.visit_command(command),
            Advice::Group(title, advice) => visitor.visit_group(title, advice),
        }
    }
}

impl From<DiagnosticTag> for DiagnosticTags {
    fn from(tag: DiagnosticTag) -> Self {
        match tag {
            DiagnosticTag::Fixable => DiagnosticTags::FIXABLE,
            DiagnosticTag::Internal => DiagnosticTags::INTERNAL,
            DiagnosticTag::Fatal => DiagnosticTags::FATAL,
            DiagnosticTag::Unnecessary => DiagnosticTags::UNNECESSARY,
            DiagnosticTag::Deprecated => DiagnosticTags::DEPRECATED,
        }
    }
}

// Custom `serde` implementation for `DiagnosticTags` as a list of `DiagnosticTag` enum
impl Serialize for DiagnosticTags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut flags = Vec::new();

        if self.contains(Self::FIXABLE) {
            flags.push(DiagnosticTag::Fixable);
        }

        if self.contains(Self::INTERNAL) {
            flags.push(DiagnosticTag::Internal);
        }

        if self.contains(Self::FATAL) {
            flags.push(DiagnosticTag::Fatal);
        }

        if self.contains(Self::UNNECESSARY) {
            flags.push(DiagnosticTag::Unnecessary);
        }

        if self.contains(Self::DEPRECATED) {
            flags.push(DiagnosticTag::Deprecated);
        }

        serializer.collect_seq(flags)
    }
}

impl<'de> Deserialize<'de> for DiagnosticTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DiagnosticTags;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "DiagnosticTags")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result = DiagnosticTags::empty();

                while let Some(item) = seq.next_element::<DiagnosticTag>()? {
                    result |= DiagnosticTags::from(item);
                }

                Ok(result)
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for DiagnosticTags {
    fn schema_name() -> String {
        String::from("DiagnosticTags")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <Vec<DiagnosticTag>>::json_schema(gen)
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use rome_text_size::{TextRange, TextSize};
    use serde_json::{from_value, json, to_value, Value};

    use crate::{
        self as rome_diagnostics,
        v2::{IntoAdvices, LogCategory, Visitor},
    };
    use rome_diagnostics_macros::Diagnostic;

    #[derive(Debug, Diagnostic)]
    #[diagnostic(
        severity = Warning,
        category = "internalError/io",
        message(
            description = "text description",
            message(<Emphasis>"markup message"</Emphasis>),
        ),
        tags(INTERNAL)
    )]
    struct TestDiagnostic {
        #[location(path)]
        path: String,
        #[location(span)]
        span: TextRange,
        #[location(source_code)]
        source_code: String,
        #[advice]
        advices: TestAdvices,
        #[verbose_advice]
        verbose_advices: TestAdvices,
    }

    impl Default for TestDiagnostic {
        fn default() -> Self {
            TestDiagnostic {
                path: String::from("path"),
                span: TextRange::new(TextSize::from(0), TextSize::from(6)),
                source_code: String::from("source_code"),
                advices: TestAdvices,
                verbose_advices: TestAdvices,
            }
        }
    }

    #[derive(Debug)]
    struct TestAdvices;

    impl IntoAdvices for TestAdvices {
        fn visit(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
            visitor.visit_log(LogCategory::Warn, &"log")?;
            Ok(())
        }
    }

    fn serialized() -> Value {
        let advices = json!([
            {
                "Log": [
                    "Warn",
                    [
                        {
                            "content": "log",
                            "elements": []
                        }
                    ]
                ]
            }
        ]);

        json!({
            "category": "internalError/io",
            "severity": "Warning",
            "description": "text description",
            "message": [
                {
                    "content": "markup message",
                    "elements": [
                        "Emphasis"
                    ]
                }
            ],
            "advices": {
                "advices": advices
            },
            "verbose_advices": {
                "advices": advices
            },
            "location": {
                "path": {
                    "File": {
                        "Path": "path"
                    }
                },
                "source_code": "source_code",
                "span": [
                    0,
                    6
                ]
            },
            "tags": [
                "Internal"
            ],
            "source": null
        })
    }

    #[test]
    fn test_serialize() {
        let diag = TestDiagnostic::default();
        let diag = super::Diagnostic::new(diag);
        let json = to_value(&diag).unwrap();

        let expected = serialized();
        assert_eq!(json, expected, "actual:\n{json:#}\nexpected:\n{expected:#}");
    }

    #[test]
    fn test_deserialize() {
        let json = serialized();
        let diag: super::Diagnostic = from_value(json).unwrap();

        let expected = TestDiagnostic::default();
        let expected = super::Diagnostic::new(expected);

        assert_eq!(
            diag, expected,
            "actual:\n{diag:#?}\nexpected:\n{expected:#?}"
        );
    }
}
