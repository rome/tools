use crate::source_map::Mapping::Mapped;
use crate::{SourceMarker, TextRange};
use rome_rowan::{Language, SyntaxNode, SyntaxNodeText, TextSize};
use schemars::Map;
use std::collections::HashMap;
use std::ops::Sub;

#[derive(Debug, Clone)]
pub struct TransformSourceMap {
    source_text: SyntaxNodeText,

    /// The mappings stored in increasing order
    deleted_ranges: Vec<DeletedRange>,

    mapped_node_ranges: HashMap<TextSize, TrimmedNodeRangeMapping>,
}

#[derive(Copy, Clone, Debug)]
enum RangePosition {
    Start,
    End,
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
        dbg!(transformed_range);

        let start_mapping = self.source_mapping(transformed_range.start(), RangePosition::Start);
        let end_mapping = self.source_mapping(transformed_range.end(), RangePosition::End);

        dbg!(start_mapping, end_mapping);

        match (start_mapping, end_mapping) {
            // `(a)`
            (
                Mapping::Adjacent {
                    source_offset: start_offset,
                    ..
                },
                Mapping::Adjacent {
                    source_offset: end_offset,
                    ..
                },
            ) => TextRange::new(start_offset, end_offset),

            (
                Mapping::Unmapped {
                    source_offset: start_offset,
                },
                Mapping::Adjacent {
                    source_offset: end_offset,
                    ..
                },
            ) => TextRange::new(start_offset, end_offset),

            // A range that is adjacent to a deleted range and spawns more than one deleted range
            // ```
            // (b + c);
            //  ^^^^^^^
            // a + (b + c)
            // ^^^^^^^^^^^
            // ```
            (
                Mapping::Adjacent {
                    source_offset: start_offset,
                    index: start_index,
                    ..
                },
                Mapping::Mapped {
                    source_offset: end_offset,
                    index: end_index,
                },
            ) if end_index - start_index > 1 => TextRange::new(start_offset, end_offset),
            (
                Mapping::Mapped {
                    source_offset: start_offset,
                    index: start_index,
                },
                Mapping::Adjacent {
                    source_offset: end_offset,
                    index: end_index,
                    ..
                },
            ) if end_index > start_index => TextRange::new(start_offset, end_offset),

            // ````
            // (b + c)
            //  ^ ^ ^
            //
            // a + (b + c) + d
            // ^^^^^^^^^^^^^^^
            // ```
            (start, end) => TextRange::new(start.source_offset(), end.source_offset()),
        }
    }

    /// Maps the trimmed range of the transformed node to the trimmed range in the source document.
    pub fn trimmed_source_range<L: Language>(&self, node: &SyntaxNode<L>) -> TextRange {
        let mut source_range = self.source_range(node.text_trimmed_range());

        dbg!(source_range);

        if let Some(mapping) = self.mapped_node_ranges.get(&source_range.start()) {
            if source_range.contains_range(mapping.trimmed_node_range) {
                source_range =
                    TextRange::new(mapping.mapped_node_range.start(), source_range.end());
            }
        }

        if let Some(mapping) = self.mapped_node_ranges.get(&source_range.end()) {
            if source_range.contains_range(mapping.trimmed_node_range) {
                source_range =
                    TextRange::new(source_range.start(), mapping.mapped_node_range.end());
            }
        }

        dbg!(source_range);

        source_range
    }

    /// Returns the source text of the trimmed range of `node`.
    pub fn trimmed_source_text<L: Language>(&self, node: &SyntaxNode<L>) -> SyntaxNodeText {
        let range = self.trimmed_source_range(node);
        self.source_text.slice(range)
    }

    fn source_mapping(&self, transformed_offset: TextSize, position: RangePosition) -> Mapping {
        dbg!(transformed_offset, position);
        let index = self
            .deleted_ranges
            .binary_search_by_key(&transformed_offset, |range| range.transformed_start());

        match index {
            // Transformed position directly falls onto a position where a deleted range starts or ends (depending on the position)
            // For example when querying: `a` in `(a)` or (a + b)`, or `b`
            Ok(index) => {
                let range = self.deleted_ranges[index];

                match position {
                    RangePosition::Start => Mapping::Adjacent {
                        source_offset: range.source_start(),
                        source_trimmed_offset: range.source_end(),
                        index,
                    },
                    // `a)`, deleted range is right after the token. That's why `source_start` is the offset
                    // that truncates the `)` and `source_end` includes it
                    RangePosition::End => Mapping::Adjacent {
                        source_offset: range.source_end(),
                        source_trimmed_offset: range.source_start(),
                        index,
                    },
                }
            }
            // The position falls outside of a position that has a leading/trailing deleted range.
            // For example, if you get the position of `+` in `(a + b)`.
            // That means, the trimmed and non-trimmed offsets are the same
            Err(index) => {
                let range = if index == 0 {
                    None
                } else {
                    self.deleted_ranges.get(index - 1)
                };

                dbg!(range);

                match range {
                    Some(range) => {
                        let transformed_delta = transformed_offset - range.transformed_start();
                        dbg!(transformed_delta);
                        let source_offset = range.source_start() + range.len() + transformed_delta;

                        Mapping::Mapped {
                            source_offset,
                            index,
                        }
                    }
                    None => Mapping::Unmapped {
                        source_offset: transformed_offset,
                    },
                }
            }
        }
    }

    pub(crate) fn map_markers(&self, markers: &[SourceMarker]) -> Vec<SourceMarker> {
        if self.deleted_ranges.is_empty() {
            Vec::from(markers)
        } else {
            Vec::from(markers)
            // // Stores the index of the last result from the mapping search.
            // let mut mapping_index = 0;
            // let mut result = Vec::with_capacity(markers.len());
            //
            // for marker in markers {
            //     let index = self.mappings[mapping_index..]
            //         .binary_search_by_key(&marker.source, |mapping| mapping.transformed_offset);
            //
            //     let delta = match index {
            //         Ok(index) => {
            //             mapping_index = index;
            //
            //             self.mappings[index].deleted_count
            //         }
            //         Err(index) => {
            //             mapping_index = index;
            //
            //             match self.mappings.get(index) {
            //                 Some(mapping) => mapping.deleted_count,
            //                 None => TextSize::default(),
            //             }
            //         }
            //     };
            //
            //     result.push(SourceMarker {
            //         source: marker.source + delta,
            //         dest: marker.dest,
            //     })
            // }
            //
            // result
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Mapping {
    /// A mapping that directly adjacent a deleted range.
    ///
    /// ## Examples
    ///
    /// Assuming a transform that removes parentheses.
    ///
    /// ```javascript
    /// (a + b)
    /// ```
    ///
    /// The identifier `a` is directly preceded by the deleted `(`, the identifier `b` is followed by the deleted `)`,
    /// and the binary expression is enclosed by the deleted `(` and `)`. On the other hand, the `+` token isn't preceded by any
    /// deleted range.
    Adjacent {
        /// The source offset for that position including the content of the deleted range.
        source_offset: TextSize,

        /// The source offset for that position excluding the content of the deleted range.
        source_trimmed_offset: TextSize,

        /// The index of the adjacent deleted range
        index: usize,
    },

    /// A mapped position that isn't adjacent to a removed range
    Mapped {
        source_offset: TextSize,
        index: usize,
    },

    /// A position that hasn't been mapped because it isn't preceded by any removed range.
    Unmapped { source_offset: TextSize },
}

impl Mapping {
    fn source_offset(&self) -> TextSize {
        match self {
            Mapping::Adjacent {
                source_trimmed_offset: source_offset,
                ..
            } => *source_offset,
            Mapping::Mapped { source_offset, .. } => *source_offset,
            Mapping::Unmapped { source_offset } => *source_offset,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct TrimmedNodeRangeMapping {
    trimmed_node_range: TextRange,
    mapped_node_range: TextRange,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DeletedRange {
    /// The range in the position of the source document
    source_range: TextRange,

    /// The absolute position in the transformed tree where that range would have been (if it hasn't been deleted).
    transformed_offset: TextSize,
}

impl DeletedRange {
    fn len(&self) -> TextSize {
        self.source_range.len()
    }

    fn source_start(&self) -> TextSize {
        self.source_range.start()
    }

    fn source_end(&self) -> TextSize {
        self.source_range.end()
    }

    fn transformed_start(&self) -> TextSize {
        self.source_range.start() - self.transformed_offset
    }

    fn map_trimmed(&self, transformed_position: TextSize) -> TextSize {
        transformed_position + self.transformed_offset + self.len()
    }
}

#[derive(Debug)]
pub struct TransformSourceMapBuilder {
    source_text: SyntaxNodeText,

    /// The mappings in increasing order by transformed offset.
    deleted_ranges: Vec<TextRange>,

    mapped_node_ranges: HashMap<TextSize, TrimmedNodeRangeMapping>,
}

impl TransformSourceMapBuilder {
    pub fn new<L: Language>(root: &SyntaxNode<L>) -> Self {
        Self {
            source_text: root.text(),
            deleted_ranges: Vec::new(),
            mapped_node_ranges: HashMap::new(),
        }
    }

    /// Adds a new mapping for a deleted character range.
    ///
    /// Mappings must be added in increasing order.
    pub fn add_deleted_range(&mut self, source_range: TextRange) {
        self.deleted_ranges.push(source_range);
    }

    pub fn add_node_range_mapping(&mut self, source_range: TextRange, to_source_range: TextRange) {
        let mapping = TrimmedNodeRangeMapping {
            trimmed_node_range: source_range,
            mapped_node_range: to_source_range,
        };

        self.mapped_node_ranges
            .insert(source_range.start(), mapping);
        self.mapped_node_ranges.insert(source_range.end(), mapping);
    }

    pub fn finish(mut self) -> TransformSourceMap {
        let mut merged_mappings = Vec::with_capacity(self.deleted_ranges.len());

        if !self.deleted_ranges.is_empty() {
            self.deleted_ranges.sort_by_key(|range| range.start());

            let mut last_mapping = DeletedRange {
                source_range: self.deleted_ranges[0],
                transformed_offset: TextSize::default(),
            };

            let mut transformed_offset = last_mapping.len();

            for range in self.deleted_ranges.drain(1..) {
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
