use super::signal::Signal;
use crate::format_element::signal::DedentMode;
use crate::prelude::*;
use crate::printer::LineEnding;
use crate::{format, write};
use crate::{
    BufferExtensions, Format, FormatContext, FormatElement, FormatOptions, FormatResult, Formatter,
    IndentStyle, LineWidth, PrinterOptions, TransformSourceMap,
};
use rome_rowan::TextSize;
use std::collections::HashMap;
use std::ops::Deref;

/// A formatted document.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Document {
    elements: Vec<FormatElement>,
}

impl From<Vec<FormatElement>> for Document {
    fn from(elements: Vec<FormatElement>) -> Self {
        Self { elements }
    }
}

impl Deref for Document {
    type Target = [FormatElement];

    fn deref(&self) -> &Self::Target {
        self.elements.as_slice()
    }
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = format!(IrFormatContext::default(), [self.elements.as_slice()])
            .expect("Formatting not to throw any FormatErrors");

        f.write_str(
            formatted
                .print()
                .expect("Expected a valid document")
                .as_code(),
        )
    }
}

#[derive(Clone, Default, Debug)]
struct IrFormatContext {
    /// The interned elements that have been printed to this point
    printed_interned_elements: HashMap<Interned, usize>,
}

impl FormatContext for IrFormatContext {
    type Options = IrFormatOptions;

    fn options(&self) -> &Self::Options {
        &IrFormatOptions
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

#[derive(Debug, Clone, Default)]
struct IrFormatOptions;

impl FormatOptions for IrFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        IndentStyle::Space(2)
    }

    fn line_width(&self) -> LineWidth {
        LineWidth(80)
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions {
            tab_width: 2,
            print_width: self.line_width().into(),
            line_ending: LineEnding::LineFeed,
            indent_style: IndentStyle::Space(2),
        }
    }
}

impl Format<IrFormatContext> for &[FormatElement] {
    fn fmt(&self, f: &mut Formatter<IrFormatContext>) -> FormatResult<()> {
        use Signal::*;

        write!(f, [ContentArrayStart])?;

        let mut signal_stack = Vec::new();
        let mut first_element = true;
        let mut in_text = false;

        let mut iter = self.iter().peekable();

        while let Some(element) = iter.next() {
            if !first_element && !in_text && !element.is_end_signal() {
                // Write a separator between every two elements
                write!(f, [text(","), soft_line_break_or_space()])?;
            }

            first_element = false;

            match element {
                element @ FormatElement::Space | element @ FormatElement::Text(_) => {
                    if !in_text {
                        write!(f, [text("\"")])?;
                    }

                    in_text = true;

                    match element {
                        FormatElement::Text(_) => f.write_element(element.clone())?,
                        FormatElement::Space => {
                            write!(f, [text(" ")])?;
                        }
                        _ => unreachable!(),
                    }

                    let is_next_text = matches!(
                        iter.peek(),
                        Some(FormatElement::Space | FormatElement::Text(_))
                    );

                    if !is_next_text {
                        write!(f, [text("\"")])?;
                        in_text = false;
                    }
                }

                FormatElement::Line(mode) => match mode {
                    LineMode::SoftOrSpace => {
                        write!(f, [text("soft_line_break_or_space")])?;
                    }
                    LineMode::Soft => {
                        write!(f, [text("soft_line_break")])?;
                    }
                    LineMode::Hard => {
                        write!(f, [text("hard_line_break")])?;
                    }
                    LineMode::Empty => {
                        write!(f, [text("empty_line")])?;
                    }
                },
                FormatElement::ExpandParent => {
                    write!(f, [text("expand_parent")])?;
                }

                FormatElement::LineSuffixBoundary => {
                    write!(f, [text("line_suffix_boundary")])?;
                }

                FormatElement::BestFitting(best_fitting) => {
                    write!(f, [text("best_fitting([")])?;
                    f.write_elements([
                        FormatElement::Signal(StartIndent),
                        FormatElement::Line(LineMode::Hard),
                    ])?;

                    for variant in best_fitting.variants() {
                        write!(f, [variant.deref(), hard_line_break()])?;
                    }

                    f.write_elements([
                        FormatElement::Signal(EndIndent),
                        FormatElement::Line(LineMode::Hard),
                    ])?;

                    write!(f, [text("])")])?;
                }

                FormatElement::Interned(interned) => {
                    let interned_elements = &mut f.context_mut().printed_interned_elements;

                    match interned_elements.get(interned).copied() {
                        None => {
                            let index = interned_elements.len();
                            interned_elements.insert(interned.clone(), index);

                            write!(
                                f,
                                [
                                    dynamic_text(
                                        &std::format!("<interned {index}>"),
                                        TextSize::default()
                                    ),
                                    space(),
                                    &interned.deref(),
                                ]
                            )?;
                        }
                        Some(reference) => {
                            write!(
                                f,
                                [dynamic_text(
                                    &std::format!("<ref interned *{reference}>"),
                                    TextSize::default()
                                )]
                            )?;
                        }
                    }
                }

                FormatElement::Signal(signal) => {
                    if signal.is_start() {
                        first_element = true;
                        signal_stack.push(signal.kind());
                    }
                    // Handle documents with mismatching start/end or superfluous end signals
                    else {
                        match signal_stack.pop() {
                            None => {
                                // Only write the end signal without any indent to ensure the output document is valid.
                                write!(
                                    f,
                                    [
                                        text("<END_SIGNAL_WITHOUT_START<"),
                                        dynamic_text(
                                            &std::format!("{:?}", signal.kind()),
                                            TextSize::default()
                                        ),
                                        text(">>"),
                                    ]
                                )?;
                                first_element = false;
                                continue;
                            }
                            Some(start_kind) if start_kind != signal.kind() => {
                                write!(
                                    f,
                                    [
                                        ContentArrayEnd,
                                        text(")"),
                                        soft_line_break_or_space(),
                                        text("ERROR<START_END_SIGNAL_MISMATCH<start: "),
                                        dynamic_text(
                                            &std::format!("{start_kind:?}"),
                                            TextSize::default()
                                        ),
                                        text(", end: "),
                                        dynamic_text(
                                            &std::format!("{:?}", signal.kind()),
                                            TextSize::default()
                                        ),
                                        text(">>")
                                    ]
                                )?;
                                first_element = false;
                                continue;
                            }
                            _ => {
                                // all ok
                            }
                        }
                    }

                    match signal {
                        StartIndent => {
                            write!(f, [text("indent(")])?;
                        }

                        StartDedent(mode) => {
                            let label = match mode {
                                DedentMode::Level => "dedent",
                                DedentMode::Root => "dedentRoot",
                            };

                            write!(f, [text(label), text("(")])?;
                        }

                        StartAlign(signal::Align(count)) => {
                            write!(
                                f,
                                [
                                    text("align("),
                                    dynamic_text(&count.to_string(), TextSize::default()),
                                    text(","),
                                    space(),
                                ]
                            )?;
                        }

                        StartLineSuffix => {
                            write!(f, [text("line_suffix(")])?;
                        }

                        StartVerbatim(_) => {
                            write!(f, [text("verbatim(")])?;
                        }

                        StartGroup(id) => {
                            write!(f, [text("group(")])?;

                            if let Some(group_id) = id {
                                write!(
                                    f,
                                    [
                                        dynamic_text(
                                            &std::format!("\"{group_id:?}\""),
                                            TextSize::default()
                                        ),
                                        text(","),
                                        space(),
                                    ]
                                )?;
                            }
                        }

                        StartIndentIfGroupBreaks(id) => {
                            write!(
                                f,
                                [
                                    text("indent_if_group_breaks("),
                                    dynamic_text(&std::format!("\"{id:?}\""), TextSize::default()),
                                    text(","),
                                    space(),
                                ]
                            )?;
                        }

                        StartConditionalContent(condition) => {
                            match condition.mode {
                                PrintMode::Flat => {
                                    write!(f, [text("if_group_fits_on_line(")])?;
                                }
                                PrintMode::Expanded => {
                                    write!(f, [text("if_group_breaks(")])?;
                                }
                            }

                            if let Some(group_id) = condition.group_id {
                                write!(
                                    f,
                                    [
                                        dynamic_text(
                                            &std::format!("\"{group_id:?}\""),
                                            TextSize::default()
                                        ),
                                        text(","),
                                        space(),
                                    ]
                                )?;
                            }
                        }

                        StartLabelled(label_id) => {
                            write!(
                                f,
                                [
                                    text("label(\""),
                                    dynamic_text(
                                        &std::format!("\"{label_id:?}\""),
                                        TextSize::default()
                                    ),
                                    text("\","),
                                    space(),
                                ]
                            )?;
                        }

                        StartFill => {
                            write!(f, [text("fill(")])?;
                        }

                        StartEntry => {
                            // handled after the match for all start signals
                        }
                        EndEntry => write!(f, [ContentArrayEnd])?,

                        EndFill
                        | EndLabelled
                        | EndConditionalContent
                        | EndIndentIfGroupBreaks
                        | EndAlign
                        | EndIndent
                        | EndGroup
                        | EndLineSuffix
                        | EndDedent
                        | EndVerbatim => {
                            write!(f, [ContentArrayEnd, text(")")])?;
                        }
                    };

                    if signal.is_start() {
                        write!(f, [ContentArrayStart])?;
                    }
                }
            }
        }

        while let Some(top) = signal_stack.pop() {
            write!(
                f,
                [
                    ContentArrayEnd,
                    text(")"),
                    soft_line_break_or_space(),
                    dynamic_text(
                        &std::format!("<START_WITHOUT_END<{top:?}>>"),
                        TextSize::default()
                    ),
                ]
            )?;
        }

        write!(f, [ContentArrayEnd])
    }
}

struct ContentArrayStart;

impl Format<IrFormatContext> for ContentArrayStart {
    fn fmt(&self, f: &mut Formatter<IrFormatContext>) -> FormatResult<()> {
        use Signal::*;

        write!(f, [text("[")])?;

        f.write_elements([
            FormatElement::Signal(StartGroup(None)),
            FormatElement::Signal(StartIndent),
            FormatElement::Line(LineMode::Soft),
        ])
    }
}

struct ContentArrayEnd;

impl Format<IrFormatContext> for ContentArrayEnd {
    fn fmt(&self, f: &mut Formatter<IrFormatContext>) -> FormatResult<()> {
        use Signal::*;
        f.write_elements([
            FormatElement::Signal(EndIndent),
            FormatElement::Line(LineMode::Soft),
            FormatElement::Signal(EndGroup),
        ])?;

        write!(f, [text("]")])
    }
}

impl FormatElements for [FormatElement] {
    fn will_break(&self) -> bool {
        use Signal::*;
        let mut ignore_depth = 0usize;

        for element in self {
            match element {
                // Line suffix
                // Ignore if any of its content breaks
                FormatElement::Signal(StartLineSuffix) => {
                    ignore_depth += 1;
                }
                FormatElement::Signal(EndLineSuffix) => {
                    ignore_depth -= 1;
                }
                FormatElement::Interned(interned) if ignore_depth == 0 => {
                    if interned.will_break() {
                        return true;
                    }
                }

                element if ignore_depth == 0 && element.will_break() => {
                    return true;
                }
                _ => continue,
            }
        }

        debug_assert_eq!(ignore_depth, 0, "Unclosed start container");

        false
    }

    fn has_label(&self, expected: LabelId) -> bool {
        self.first()
            .map_or(false, |element| element.has_label(expected))
    }

    fn start_signal(&self, kind: SignalKind) -> Option<&Signal> {
        // Assert that the document ends at a signal with the specified kind;
        let _ = self.end_signal(kind)?;

        fn traverse_slice<'a>(
            slice: &'a [FormatElement],
            kind: SignalKind,
            depth: &mut usize,
        ) -> Option<&'a Signal> {
            for element in slice.iter().rev() {
                match element {
                    FormatElement::Signal(signal) if signal.kind() == kind => {
                        if signal.is_start() {
                            if *depth == 0 {
                                // Invalid document
                                return None;
                            } else if *depth == 1 {
                                return Some(signal);
                            } else {
                                *depth -= 1;
                            }
                        } else {
                            *depth += 1;
                        }
                    }
                    FormatElement::Interned(interned) => {
                        match traverse_slice(interned, kind, depth) {
                            Some(start) => {
                                return Some(start);
                            }
                            // Reached end or invalid document
                            None if *depth == 0 => {
                                return None;
                            }
                            _ => {
                                // continue with other elements
                            }
                        }
                    }
                    _ => {}
                }
            }

            None
        }

        let mut depth = 0usize;

        traverse_slice(self, kind, &mut depth)
    }

    fn end_signal(&self, kind: SignalKind) -> Option<&Signal> {
        self.last().and_then(|element| element.end_signal(kind))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::SimpleFormatContext;
    use crate::{format, format_args, write};

    #[test]
    fn display_elements() {
        let formatted = format!(
            SimpleFormatContext::default(),
            [format_with(|f| {
                write!(
                    f,
                    [group(&format_args![
                        text("("),
                        soft_block_indent(&format_args![
                            text("Some longer content"),
                            space(),
                            text("That should ultimately break"),
                        ])
                    ])]
                )
            })]
        )
        .unwrap();

        let document = formatted.into_document();

        assert_eq!(
            &std::format!("{document}"),
            r#"[
  group([
    "(",
    indent([
      soft_line_break,
      "Some longer content That should ultimately break"
    ]),
    soft_line_break
  ])
]"#
        );
    }

    #[test]
    fn display_invalid_document() {
        use Signal::*;

        let document = Document::from(vec![
            FormatElement::Text(Text::Static { text: "[" }),
            FormatElement::Signal(StartGroup(None)),
            FormatElement::Signal(StartIndent),
            FormatElement::Line(LineMode::Soft),
            FormatElement::Text(Text::Static { text: "a" }),
            // Close group instead of indent
            FormatElement::Signal(EndGroup),
            FormatElement::Line(LineMode::Soft),
            FormatElement::Signal(EndIndent),
            FormatElement::Text(Text::Static { text: "]" }),
            // End signal without start
            FormatElement::Signal(EndIndent),
            // Start signal without an end
            FormatElement::Signal(StartIndent),
        ]);

        assert_eq!(
            &std::format!("{document}"),
            r#"[
  "[",
  group([
    indent([soft_line_break, "a"])
    ERROR<START_END_SIGNAL_MISMATCH<start: Indent, end: Group>>,
    soft_line_break
  ])
  ERROR<START_END_SIGNAL_MISMATCH<start: Group, end: Indent>>,
  "]"<END_SIGNAL_WITHOUT_START<Indent>>,
  indent([])
  <START_WITHOUT_END<Indent>>
]"#
        );
    }
}
