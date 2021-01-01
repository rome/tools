/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BaseTokens,
	ComplexNode,
	SimpleNode,
	SimpleToken,
	StringToken,
	ValueNode,
} from "@internal/parser-core";

//# Tokens
export type Tokens = BaseTokens & {
	Exclamation: SimpleToken<"Exclamation">;
	Star: SimpleToken<"Star">;
	DoubleStar: SimpleToken<"DoubleStar">;
	Word: StringToken<"Word">;
	Separator: SimpleToken<"Separator">;
	Comment: StringToken<"Comment">;
	EOL: SimpleToken<"EOL">;
};

//# Nodes
export type WordNode = ValueNode<"Word", string>;

export type WildcardNode = SimpleNode<"Wildcard">;

export type PatternPartNode = WildcardNode | WordNode;

export type PatternParts = PatternPartNode[];

export type PatternWordSegmentNode = ComplexNode<
	"Segment",
	{
		parts: PatternParts;
	}
>;

export type PatternWildcardSegmentNode = SimpleNode<"WildcardSegment">;

export type PatternSegmentNode =
	| PatternWordSegmentNode
	| PatternWildcardSegmentNode;

export type PatternSegments = PatternSegmentNode[];

export type PathPatternNode = ComplexNode<
	"PathPattern",
	{
		negate: boolean;
		root: boolean;
		segments: PatternSegments;
	}
>;

export type CommentNode = ValueNode<"Comment", string>;

export type PathPatterns = PathPattern[];

export type PathPattern = PathPatternNode | CommentNode;
