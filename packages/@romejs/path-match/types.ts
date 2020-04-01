/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  BaseTokens,
  SimpleNode,
  ValueNode,
  ComplexNode,
  SimpleToken,
  ValueToken,
} from '@romejs/parser-core';

//# Tokens
export type Tokens = BaseTokens & {
  Exclamation: SimpleToken<'Exclamation'>;
  Star: SimpleToken<'Star'>;
  DoubleStar: SimpleToken<'DoubleStar'>;
  Word: ValueToken<'Word', string>;
  Separator: SimpleToken<'Separator'>;
  Hash: SimpleToken<'Hash'>;
};

//# Nodes
export type WordNode = ValueNode<'Word', string>;

export type WildcardNode = SimpleNode<'Wildcard'>;

export type PatternPartNode = WildcardNode | WordNode;

export type PatternParts = Array<PatternPartNode>;

export type PatternWordSegmentNode = ComplexNode<'Segment', {
  parts: PatternParts;
}>;

export type PatternWildcardSegmentNode = SimpleNode<'WildcardSegment'>;

export type PatternSegmentNode =
  | PatternWordSegmentNode
  | PatternWildcardSegmentNode;

export type PatternSegments = Array<PatternSegmentNode>;

export type PathPatternNode = ComplexNode<'PathPattern', {
  comment: string;
  negate: boolean;
  root: boolean;
  segments: PatternSegments;
}>;
