use crate::{SourceMarker, TextRange};
use rome_rowan::{Language, SyntaxNode, SyntaxNodeText, TextSize};
use std::ops::Sub;

#[derive(Debug, Clone)]
pub struct TransformSourceMap {
    source_text: SyntaxNodeText,

    /// The mappings stored in increasing order
    mappings: Vec<DeletedRange>,
}

impl TransformSourceMap {
    /// Creates a source map for a unchanged tree that has no mappings.
    pub fn empty<L: Language>(source: &SyntaxNode<L>) -> Self {
        Self {
            source_text: source.text(),
            mappings: Vec::new(),
        }
    }

    fn find_mapping(&self, transformed_position: TextSize) -> Option<&DeletedRange> {
        let index = self
            .mappings
            .binary_search_by_key(&transformed_position, |mapping| mapping.transformed_start());

        match index {
            Ok(index) => Some(&self.mappings[index]),
            Err(index) => {
                if index == 0 {
                    None
                } else {
                    self.mappings.get(index - 1)
                }
            }
        }
    }

    pub fn resolve_range(&self, transformed_range: TextRange) -> TextRange {
        dbg!(transformed_range);
        match self.find_mapping(transformed_range.start()) {
            Some(start_mapping) => {
                let is_at_range_start =
                    transformed_range.start() == start_mapping.transformed_start();

                let end_mapping = if is_at_range_start {
                    self.find_mapping(transformed_range.end()).unwrap()
                } else if transformed_range.is_empty() {
                    start_mapping
                } else {
                    // Find the mapping up to but not including the end position
                    self.find_mapping(transformed_range.end().sub(TextSize::from(1)))
                        .unwrap()
                };

                let mapping_range = TextRange::new(
                    start_mapping.transformed_start(),
                    end_mapping.transformed_start(),
                );

                dbg!(mapping_range);

                if transformed_range.contains_range(mapping_range) && start_mapping != end_mapping {
                    let end_offset = transformed_range.end() - mapping_range.end();
                    TextRange::new(
                        start_mapping.source_start(),
                        end_mapping.source_end() + end_offset,
                    )
                } else {
                    TextRange::new(
                        start_mapping.map_trimmed(transformed_range.start()),
                        end_mapping.map_trimmed(transformed_range.end()),
                    )
                }
            }
            None => match self.find_mapping(transformed_range.end()) {
                Some(end_mapping) => {
                    if transformed_range.end() == end_mapping.transformed_start() {
                        TextRange::new(transformed_range.start(), end_mapping.source_end())
                    } else {
                        TextRange::new(
                            transformed_range.start(),
                            end_mapping.map_trimmed(transformed_range.end()),
                        )
                    }
                }
                None => transformed_range,
            },
        }
    }

    pub fn resolve_text(&self, transformed_range: TextRange) -> String {
        let range = self.resolve_range(transformed_range);

        self.source_text.slice(range).to_string()
    }

    pub(crate) fn map_markers(&self, markers: &[SourceMarker]) -> Vec<SourceMarker> {
        if self.mappings.is_empty() {
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
}

impl TransformSourceMapBuilder {
    pub fn new<L: Language>(root: &SyntaxNode<L>) -> Self {
        Self {
            source_text: root.text(),
            deleted_ranges: Vec::new(),
        }
    }

    /// Adds a new mapping for a deleted character range.
    ///
    /// Mappings must be added in increasing order.
    pub fn add_deleted_range(&mut self, source_range: TextRange) {
        self.deleted_ranges.push(source_range);
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
            mappings: merged_mappings,
        }
    }
}
