/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BaseTokens,
	ComplexNode,
	NumberToken,
	SimpleNode,
	SimpleToken,
	StringToken,
	ValueToken,
} from "@internal/parser-core";

export enum SemverModifier {
	MAJOR,
	MINOR,
	PATCH,
}

export type SemverVersionPrereleaseParts = Array<number | string>;

// 1.2, 1, 1.*.2
export type SemverWildcardVersion = ComplexNode<
	"SemverWildcardVersion",
	{
		major: undefined | number;
		minor: undefined | number;
		patch: undefined | number;
		prerelease: SemverVersionPrereleaseParts;
		build: SemverVersionPrereleaseParts;
	}
>;

// 1.2.3
export type SemverVersion = ComplexNode<
	"SemverAbsoluteVersion",
	{
		major: number;
		minor: number;
		patch: number;
		prerelease: SemverVersionPrereleaseParts;
		build: SemverVersionPrereleaseParts;
	}
>;

// 1.2.x, 1.X, 1.2.*, *
export type SemverWildcard = SimpleNode<"SemverWildcard">;

// >=1.2.3
export type SemverComparatorOperator =
	| "<"
	| ">"
	| ">="
	| "<="
	| "~>"
	| "^"
	| "~"
	| "=";

export type SemverComparator = ComplexNode<
	"SemverComparator",
	{
		operator: SemverComparatorOperator;
		version: SemverWildcard | SemverWildcardVersion | SemverVersion;
	}
>;

// 1.2.3 || 4.5.6
export type SemverLogicalOr = ComplexNode<
	"SemverLogicalOr",
	{
		left: SemverRange;
		right: SemverRange;
	}
>;

// 1.2.3 4.5.6
export type SemverLogicalAnd = ComplexNode<
	"SemverLogicalAnd",
	{
		left: SemverRange;
		right: SemverRange;
	}
>;

// 1.2.3 - 2.3.4
export type SemverVersionRange = ComplexNode<
	"SemverVersionRange",
	{
		left: SemverWildcard | SemverWildcardVersion | SemverVersion;
		right: SemverWildcard | SemverWildcardVersion | SemverVersion;
	}
>;

export type SemverRange =
	| SemverLogicalAnd
	| SemverVersionRange
	| SemverLogicalOr
	| SemverComparator
	| SemverWildcard
	| SemverWildcardVersion
	| SemverVersion;

export type Tokens = BaseTokens & {
	Space: SimpleToken<"Space">;
	Number: NumberToken<"Number">;
	Word: StringToken<"Word">;
	Dash: SimpleToken<"Dash">;
	RangeDash: SimpleToken<"RangeDash">;
	Plus: SimpleToken<"Plus">;
	Star: SimpleToken<"Star">;
	Operator: ValueToken<"Operator", SemverComparatorOperator>;
	Dot: SimpleToken<"Dot">;
	Pipe: SimpleToken<"Pipe">;
};
