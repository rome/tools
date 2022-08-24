use crate::{SourceMarker, TextRange};
use rome_rowan::{Language, SyntaxNode, SyntaxNodeText, TextSize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TransformSourceMap {
    source_text: SyntaxNodeText,

    /// The mappings stored in increasing order
    deleted_ranges: Vec<DeletedRange>,

    mapped_node_ranges: HashMap<TextSize, TrimmedNodeRangeMapping>,
}

impl TransformSourceMap {
    /// Creates a source map for a unchanged tree that has no mappings.
    pub fn empty<L: Language>(source: &SyntaxNode<L>) -> Self {
        Self {
            source_text: source.text(),
            deleted_ranges: Vec::new(),
            mapped_node_ranges: HashMap::default(),
        }
    }

    /// Returns the text of the source document as it was before the transformation.
    pub fn text(&self) -> &SyntaxNodeText {
        &self.source_text
    }

    /// Maps a range of the transformed document to a range in the source document.
    pub fn source_range(&self, transformed_range: TextRange) -> TextRange {
        TextRange::new(
            self.source_offset(transformed_range.start(), RangePosition::Start),
            self.source_offset(transformed_range.end(), RangePosition::End),
        )
    }

    /// Maps the trimmed range of the transformed node to the trimmed range in the source document.
    pub fn trimmed_source_range<L: Language>(&self, node: &SyntaxNode<L>) -> TextRange {
        let source_range = self.source_range(node.text_trimmed_range());

        let mut mapped_range = source_range;

        loop {
            let mut widened = false;

            let start_mapping = self.mapped_node_ranges.get(&mapped_range.start());
            if let Some(mapping) = start_mapping {
                if mapped_range.contains_range(mapping.original_range) {
                    mapped_range =
                        TextRange::new(mapping.extended_range.start(), mapped_range.end());
                    widened = true;
                }
            }

            let end_mapping = self.mapped_node_ranges.get(&mapped_range.end());
            if let Some(mapping) = end_mapping {
                if mapped_range.contains_range(mapping.original_range) {
                    mapped_range =
                        TextRange::new(mapped_range.start(), mapping.extended_range.end());
                    widened = true;
                }
            }

            if !widened {
                break;
            }
        }

        mapped_range
    }

    /// Returns the source text of the trimmed range of `node`.
    pub fn trimmed_source_text<L: Language>(&self, node: &SyntaxNode<L>) -> SyntaxNodeText {
        let range = self.trimmed_source_range(node);
        self.source_text.slice(range)
    }

    fn source_offset(&self, transformed_offset: TextSize, position: RangePosition) -> TextSize {
        let index = self
            .deleted_ranges
            .binary_search_by_key(&transformed_offset, |range| range.transformed_start());

        let range = match index {
            Ok(index) => Some(&self.deleted_ranges[index]),
            Err(index) => {
                if index == 0 {
                    None
                } else {
                    self.deleted_ranges.get(index - 1)
                }
            }
        };

        self.source_offset_with_range(transformed_offset, position, range)
    }

    fn source_offset_with_range(
        &self,
        transformed_offset: TextSize,
        position: RangePosition,
        deleted_range: Option<&DeletedRange>,
    ) -> TextSize {
        match deleted_range {
            Some(range) => {
                // Transformed position directly falls onto a position where a deleted range starts or ends (depending on the position)
                // For example when querying: `a` in `(a)` or (a + b)`, or `b`
                if range.transformed_start() == transformed_offset {
                    match position {
                        RangePosition::Start => range.source_end(),
                        // `a)`, deleted range is right after the token. That's why `source_start` is the offset
                        // that truncates the `)` and `source_end` includes it
                        RangePosition::End => range.source_start(),
                    }
                }
                // The position falls outside of a position that has a leading/trailing deleted range.
                // For example, if you get the position of `+` in `(a + b)`.
                // That means, the trimmed and non-trimmed offsets are the same
                else {
                    let transformed_delta = transformed_offset - range.transformed_start();
                    range.source_start() + range.len() + transformed_delta
                }
            }
            None => transformed_offset,
        }
    }

    pub fn map_markers(&self, markers: &mut [SourceMarker]) {
        if !self.deleted_ranges.is_empty() {
            // Stores the index of the last result from the mapping search.
            let mut current_range: Option<DeletedRange> = None;
            let mut ranges = self.deleted_ranges.iter();
            let mut next_range = ranges.next().copied();

            for marker in markers {
                while let Some(range) = next_range {
                    if range.transformed_start() > marker.source {
                        break;
                    }

                    current_range = std::mem::replace(&mut next_range, ranges.next().copied());
                }

                let source = self.source_offset_with_range(
                    marker.source,
                    RangePosition::Start,
                    current_range.as_ref(),
                );

                marker.source = source;
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct TrimmedNodeRangeMapping {
    /// The
    original_range: TextRange,
    extended_range: TextRange,
}

#[derive(Copy, Clone, Debug)]
enum RangePosition {
    Start,
    End,
}

/// Stores the information about a range in the source document that isn't present in the transformed document
/// and provides means to map the transformed position back to the source position.
///
/// # Examples
///
/// ```javascript
/// (a + b)
/// ```
///
/// A transform that removes the parentheses from the above expression removes the ranges `0..1` (`(` token)
/// and `6..7` (`)` token) and the source map creates one [DeletedRange] for each:
///
/// ```text
/// DeletedRange {
///     source_range: 0..1,
///     transformed_offset: 0,
/// },
/// DeletedRange {
///     source_range: 6..7,
///     transformed_offset: 1,
/// }
/// ```
///
/// The first range indicates that the range `0..1` for the `(` token has been removed. The second range
/// indicates that the range `6..7` for the `)` has been removed and it stores that, up to this point,
/// but not including, 1 more byte has been removed.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DeletedRange {
    /// The range in the source document of the bytes that have been omitted from the transformed document.
    source_range: TextRange,

    /// The accumulated count of all removed bytes up to (but not including) the start of this range.
    transformed_offset: TextSize,
}

impl DeletedRange {
    /// The number of deleted characters starting from [source offset](DeletedRange::source_start).
    fn len(&self) -> TextSize {
        self.source_range.len()
    }

    /// The start position in bytes in the source document of the omitted sequence in the transformed document.
    fn source_start(&self) -> TextSize {
        self.source_range.start()
    }

    /// The end position in bytes in the source document of the omitted sequence in the transformed document.
    fn source_end(&self) -> TextSize {
        self.source_range.end()
    }

    /// Returns the byte position of [DeleteRange::source_start] in the transformed document.
    fn transformed_start(&self) -> TextSize {
        self.source_range.start() - self.transformed_offset
    }
}

/// Builder for creating a source map.
#[derive(Debug)]
pub struct TransformSourceMapBuilder {
    /// The original source text of the tree before it was transformed.
    source_text: SyntaxNodeText,

    /// The mappings in increasing order by transformed offset.
    deleted_ranges: Vec<TextRange>,

    /// The keys are a position in the source map where a trimmed node starts or ends.
    /// The values are the metadata about a trimmed node range
    mapped_node_ranges: HashMap<TextSize, TrimmedNodeRangeMapping>,
}

impl TransformSourceMapBuilder {
    /// Creates a new builder for a source map that maps positions back to the passed `root` tree.
    pub fn new<L: Language>(root: &SyntaxNode<L>) -> Self {
        Self {
            source_text: root.text(),
            deleted_ranges: Vec::new(),
            mapped_node_ranges: HashMap::new(),
        }
    }

    /// Adds a new mapping for a deleted character range.
    pub fn add_deleted_range(&mut self, source_range: TextRange) {
        self.deleted_ranges.push(source_range);
    }

    /// Adds a mapping to widen a nodes trimmed range.
    ///
    /// The formatter uses the trimmed range when formatting a node in verbatim either because the node
    /// failed to format because of a syntax error or because it's formatting is suppressed with a `rome-ignore format:` comment.
    ///
    /// This method adds a mapping to widen a nodes trimmed range to enclose another range instead. This is
    /// e.g. useful when removing parentheses around expressions where `(/* comment */ a /* comment */)` because
    /// the trimmed range of `a` should now enclose the full range including the `(` and `)` tokens to ensure
    /// that the parentheses are retained when printing that node in verbatim style.
    pub fn extend_trimmed_node_range(
        &mut self,
        original_range: TextRange,
        extended_range: TextRange,
    ) {
        let mapping = TrimmedNodeRangeMapping {
            original_range,
            extended_range,
        };

        self.mapped_node_ranges
            .insert(original_range.start(), mapping);
        self.mapped_node_ranges
            .insert(original_range.end(), mapping);
    }

    /// Creates a source map has a complexity of `O(log(n))` to look up a single offset mapping.
    pub fn finish(mut self) -> TransformSourceMap {
        let mut merged_mappings = Vec::with_capacity(self.deleted_ranges.len());

        if !self.deleted_ranges.is_empty() {
            self.deleted_ranges.sort_by_key(|range| range.start());

            let mut last_mapping = DeletedRange {
                // SAFETY: Safe because of the not empty check above
                source_range: self.deleted_ranges[0],
                transformed_offset: TextSize::default(),
            };

            let mut transformed_offset = last_mapping.len();

            for range in self.deleted_ranges.drain(1..) {
                // Merge adjacent ranges to ensure there's only ever a single mapping starting at the same transformed offset.
                if last_mapping.source_range.end() == range.start() {
                    last_mapping.source_range = last_mapping.source_range.cover(range);
                } else {
                    merged_mappings.push(last_mapping);

                    last_mapping = DeletedRange {
                        source_range: range,
                        transformed_offset,
                    };
                }
                transformed_offset += range.len();
            }

            merged_mappings.push(last_mapping);
        }

        TransformSourceMap {
            source_text: self.source_text,
            deleted_ranges: merged_mappings,
            mapped_node_ranges: self.mapped_node_ranges,
        }
    }
}
