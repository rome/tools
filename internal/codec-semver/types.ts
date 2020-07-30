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
	ValueToken,
} from "@internal/parser-core";

// PARSER
export type VersionPrereleaseParts = Array<number | string>;

// 1.2, 1, 1.*.2
export type WildcardVersionNode = ComplexNode<
	"WildcardVersion",
	{
		major: undefined | number;
		minor: undefined | number;
		patch: undefined | number;
		prerelease: VersionPrereleaseParts;
		build: VersionPrereleaseParts;
	}
>;

// 1.2.3
export type AbsoluteVersionNode = ComplexNode<
	"AbsoluteVersion",
	{
		major: number;
		minor: number;
		patch: number;
		prerelease: VersionPrereleaseParts;
		build: VersionPrereleaseParts;
	}
>;

// union to treat these as the same
export type VersionNode = WildcardVersionNode | AbsoluteVersionNode;

// 1.2.x, 1.X, 1.2.*, *
export type WildcardNode = SimpleNode<"Wildcard">;

// >=1.2.3
export type ComparatorOperator =
	| "<"
	| ">"
	| ">="
	| "<="
	| "~>"
	| "^"
	| "~"
	| "=";

export type ComparatorNode = ComplexNode<
	"Comparator",
	{
		operator: ComparatorOperator;
		version: WildcardNode | VersionNode;
	}
>;

// 1.2.3 || 4.5.6
export type LogicalOrNode = ComplexNode<
	"LogicalOr",
	{
		left: RangeNode;
		right: RangeNode;
	}
>;

// 1.2.3 4.5.6
export type LogicalAndNode = ComplexNode<
	"LogicalAnd",
	{
		left: RangeNode;
		right: RangeNode;
	}
>;

// 1.2.3 - 2.3.4
export type VersionRangeNode = ComplexNode<
	"VersionRange",
	{
		left: WildcardNode | VersionNode;
		right: WildcardNode | VersionNode;
	}
>;

export type RangeNode =
	| LogicalAndNode
	| VersionRangeNode
	| LogicalOrNode
	| ComparatorNode
	| WildcardNode
	| VersionNode;

// TOKENS
export type Tokens = BaseTokens & {
	Space: SimpleToken<"Space">;
	Number: ValueToken<"Number", number>;
	Word: ValueToken<"Word", string>;
	Dash: SimpleToken<"Dash">;
	RangeDash: SimpleToken<"RangeDash">;
	Plus: SimpleToken<"Plus">;
	Star: SimpleToken<"Star">;
	Operator: ValueToken<"Operator", ComparatorOperator>;
	Dot: SimpleToken<"Dot">;
	Pipe: SimpleToken<"Pipe">;
};

// Types for public API
export type UserVersion = AbsoluteVersionNode | string;

export type UserRange = RangeNode | string;

export type UserVersions = Array<UserVersion>;
