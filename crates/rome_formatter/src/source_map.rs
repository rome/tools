use crate::{Printed, SourceMarker, TextRange};
use rome_rowan::{Language, SyntaxNode, SyntaxNodeText, TextSize};
use std::cmp::Ordering;
use std::collections::HashMap;

/// A source map for mapping positions of a pre-processed tree back to the locations in the source tree.
///
/// This is not a generic purpose source map but instead focused on supporting the case where
/// a language removes or re-orders nodes that would otherwise complicate the formatting logic.
/// A common use case for pre-processing is the removal all parenthesized nodes
/// because parenthesized nodes complicate testing if a child or parent is of a specific kind as they need to be ignored.
///
/// This source map implementation only must support removing tokens or re-structuring nodes
/// without changing the order of the tokens in the tree (requires no source map).
///
/// ## Position Mapping
///
/// The source map internally tracks all the ranges that have been deleted from the source code sorted by the start of the deleted range.
/// It further stores the absolute count of deleted bytes preceding a range. The deleted range together
/// with the absolute count allows to re-compute the source location for every transformed location
/// and has the benefit that it requires significantly fewer memory
/// than source maps that use a source to destination position marker for every token.
///
/// ## Map Node Ranges
///
/// Only having the deleted ranges to resolve the original text of a node isn't sufficient.
/// Resolving the original text of a node is needed when formatting a node as verbatim, either because
/// formatting the node failed because of a syntax error, or formatting is suppressed with a `rome-ignore format:` comment.
///
/// ```text
/// // Source           // Transformed
///  (a+b) + (c + d)   a + b + c + d;
/// ```
///
/// Using the above example, the following source ranges should be returned when quering with the transformed ranges:
///
/// * `a` -> `a`: Should not include the leading `(`
/// * `b` -> `b`: Should not include the trailing `)`
/// * `a + b` -> `(a + b)`: Should include the leading `(` and trailing `)`.
/// * `a + b + c + d -> `(a + b) + (c + d)`: Should include the leading `(` and `)` trailing `)` because the expression statement
///   fully encloses the `a + b` and `c + d` nodes.
///
/// This is why the source map also tracks the mapped trimmed ranges for every node.
#[derive(Debug, Clone)]
pub struct TransformSourceMap {
    source_text: SyntaxNodeText,

    /// The mappings stored in increasing order
    deleted_ranges: Vec<DeletedRange>,

    /// Key: Start or end position of node for which the trimmed range should be extended
    /// Value: The trimmed range.
    mapped_node_ranges: HashMap<TextSize, TrimmedNodeRangeMapping>,
}

impl TransformSourceMap {
    /// Returns the text of the source document as it was before the transformation.
    pub fn text(&self) -> &SyntaxNodeText {
        &self.source_text
    }

    /// Maps a range of the transformed document to a range in the source document.
    ///
    /// Complexity: `O(log(n))`
    pub fn source_range(&self, transformed_range: TextRange) -> TextRange {
        TextRange::new(
            self.source_offset(transformed_range.start(), RangePosition::Start),
            self.source_offset(transformed_range.end(), RangePosition::End),
        )
    }

    /// Maps the trimmed range of the transformed node to the trimmed range in the source document.
    ///
    /// Average Complexity: `O(log(n))`
    pub fn trimmed_source_range<L: Language>(&self, node: &SyntaxNode<L>) -> TextRange {
        let source_range = self.source_range(node.text_trimmed_range());

        let mut mapped_range = source_range;

        loop {
            let mut widened = false;

            let start_mapping = self.mapped_node_ranges.get(&mapped_range.start());
            if let Some(mapping) = start_mapping {
                // If the queried node fully encloses the original range of the node, then extend the range
                if mapped_range.contains_range(mapping.original_range) {
                    mapped_range =
                        TextRange::new(mapping.extended_range.start(), mapped_range.end());
                    widened = true;
                }
            }

            let end_mapping = self.mapped_node_ranges.get(&mapped_range.end());
            if let Some(mapping) = end_mapping {
                // If the queried node fully encloses the original range of the node, then extend the range
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
                debug_assert!(
                    range.transformed_start() <= transformed_offset,
                    "Transformed start {:?} must be less than or equal to transformed offset {:?}.",
                    range.transformed_start(),
                    transformed_offset
                );
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

    /// Maps the source map information of `printed` from the transformed positions to the source positions.
    pub fn map_printed(&self, mut printed: Printed) -> Printed {
        self.map_markers(&mut printed.sourcemap);
        self.map_verbatim_ranges(&mut printed.verbatim_ranges);

        printed
    }

    /// Maps the printers source map marker to the source positions.
    fn map_markers(&self, markers: &mut [SourceMarker]) {
        if self.deleted_ranges.is_empty() {
            return;
        }

        let mut previous_marker: Option<SourceMarker> = None;
        let mut next_range_index = 0;

        for marker in markers {
            // It's not guaranteed that markers are sorted by source location (line suffix comments).
            // It can, therefore, be necessary to navigate backwards again.
            // In this case, do a binary search for the index of the next deleted range (`O(log(n)`).
            let out_of_order_marker =
                previous_marker.map_or(false, |previous| previous.source > marker.source);

            if out_of_order_marker {
                let index = self
                    .deleted_ranges
                    .binary_search_by_key(&marker.source, |range| range.transformed_start());

                match index {
                    // Direct match
                    Ok(index) => {
                        next_range_index = index + 1;
                    }
                    Err(index) => next_range_index = index,
                }
            } else {
                // Find the range for this mapping. In most cases this is a no-op or only involves a single step
                // because markers are most of the time in increasing source order.
                while next_range_index < self.deleted_ranges.len() {
                    let next_range = &self.deleted_ranges[next_range_index];

                    if next_range.transformed_start() > marker.source {
                        break;
                    }

                    next_range_index += 1;
                }
            }

            previous_marker = Some(*marker);

            let current_range = if next_range_index == 0 {
                None
            } else {
                self.deleted_ranges.get(next_range_index - 1)
            };

            let source =
                self.source_offset_with_range(marker.source, RangePosition::Start, current_range);

            marker.source = source;
        }
    }

    fn map_verbatim_ranges(&self, ranges: &mut [TextRange]) {
        for range in ranges {
            *range = self.source_range(*range)
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct TrimmedNodeRangeMapping {
    /// The original trimmed range of the node.
    ///
    /// ```javascript
    /// (a + b)
    /// ```
    ///
    /// `1..6` `a + b`
    original_range: TextRange,

    /// The range to which the trimmed range of the node should be extended
    /// ```javascript
    /// (a + b)
    /// ```
    ///
    /// `0..7` for `a + b` if its range should also include the parenthesized range.
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
    fn new(source_range: TextRange, transformed_offset: TextSize) -> Self {
        debug_assert!(source_range.start() >= transformed_offset);

        Self {
            source_range,
            transformed_offset,
        }
    }

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
            self.deleted_ranges
                .sort_by(|a, b| match a.start().cmp(&b.start()) {
                    Ordering::Equal => a.end().cmp(&b.end()),
                    ordering => ordering,
                });

            let mut last_mapping = DeletedRange::new(
                // SAFETY: Safe because of the not empty check above
                self.deleted_ranges[0],
                TextSize::default(),
            );

            let mut transformed_offset = last_mapping.len();

            for range in self.deleted_ranges.drain(1..) {
                // Merge adjacent ranges to ensure there's only ever a single mapping starting at the same transformed offset.
                if last_mapping.source_range.end() == range.start() {
                    last_mapping.source_range = last_mapping.source_range.cover(range);
                } else {
                    merged_mappings.push(last_mapping);

                    last_mapping = DeletedRange::new(range, transformed_offset);
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
