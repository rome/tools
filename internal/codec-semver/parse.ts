/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AbsoluteVersionNode,
	ComparatorNode,
	ComparatorOperator,
	LogicalAndNode,
	LogicalOrNode,
	RangeNode,
	Tokens,
	VersionNode,
	VersionPrereleaseParts,
	VersionRangeNode,
	WildcardNode,
} from "./types";
import {
	ParserCore,
	ParserOptions,
	TokenValues,
	createParser,
	isAlpha,
	isDigit,
} from "@internal/parser-core";
import {ob1Add} from "@internal/ob1";
import {descriptions} from "@internal/diagnostics";

export type SemverParserOptions = ParserOptions & {
	loose?: boolean;
};

type Meta = {
	mode: "version" | "range";
};

type SemverParserTypes = {
	tokens: Tokens;
	state: {};
	options: SemverParserOptions;
	meta: Meta;
};
type SemverParser = ParserCore<SemverParserTypes>;

const createSemverParser = createParser<SemverParserTypes>({
	diagnosticCategory: "parse/semver",
	normalizeInput: (input) => input.trimRight(),
	tokenize(parser, index) {
		const char = parser.getInputCharOnly(index);
		const nextChar = parser.getInputCharOnly(index, 1);

		if (
			(char === "<" && nextChar === "=") ||
			(char === ">" && nextChar === "=") ||
			(char === "~" && nextChar === ">")
		) {
			// @ts-ignore: TS doesn't infer the possible combinations
			const value: ComparatorOperator = char + nextChar;
			return parser.finishValueToken("Operator", value, ob1Add(index, 2));
		}

		if (
			char === "^" ||
			char === "<" ||
			char === ">" ||
			char === "~" ||
			char === "="
		) {
			const op: ComparatorOperator = char;
			return parser.finishValueToken("Operator", op);
		}

		if (char === "|" && nextChar === "|") {
			return parser.finishToken("Pipe", ob1Add(index, 2));
		}

		if (char === "*") {
			return parser.finishToken("Star");
		}

		if (
			parser.getInputCharOnly(index, -1) === " " &&
			char === "-" &&
			nextChar === " "
		) {
			return parser.finishToken("RangeDash");
		}

		if (char === "-") {
			return parser.finishToken("Dash");
		}

		if (char === "+") {
			return parser.finishToken("Plus");
		}

		if (char === ".") {
			return parser.finishToken("Dot");
		}

		if (isDigit(char)) {
			const [value] = parser.readInputFrom(index, isDigit);
			return parser.finishValueToken(
				"Number",
				Number(value),
				ob1Add(index, value.length),
			);
		}

		if (isAlpha(char)) {
			const [value] = parser.readInputFrom(index, isAlpha);
			return parser.finishValueToken("Word", value, ob1Add(index, value.length));
		}

		if (char === " " || char === "\t") {
			return parser.finishToken("Space");
		}

		// Unknown character
		return undefined;
	},
});

// Remove all subsequent space tokens
function eatSpaceToken(parser: SemverParser) {
	while (parser.eatToken("Space") !== undefined) {
		// empty
	}
}

function parseVersionOrWildcard(
	parser: SemverParser,
): WildcardNode | VersionNode {
	const startPos = parser.getPosition();
	const startToken = parser.getToken();
	const version = parseVersion(parser);

	// We should return a bare wildcard when parsed in a version position if there was nothing else attached
	if (
		isWildcardToken(parser, startToken) &&
		version.minor === undefined &&
		version.patch === undefined &&
		version.prerelease.length === 0 &&
		version.build.length === 0
	) {
		return {
			type: "Wildcard",
			loc: parser.finishLoc(startPos),
		};
	}

	return version;
}

function parseVersion(parser: SemverParser): VersionNode {
	const startPos = parser.getPosition();
	const startToken = parser.getToken();

	if (isVersionCharacter(parser, startToken)) {
		parser.nextToken();
	}

	const major = parseVersionNumber(parser);
	let minor = undefined;
	let patch = undefined;

	if (parser.eatToken("Dot")) {
		minor = parseVersionNumber(parser);
	} else if (parser.meta.mode === "version") {
		throw parser.unexpected({
			description: descriptions.SEMVER.MISSING_MINOR_VERSION,
		});
	}

	if (parser.eatToken("Dot")) {
		patch = parseVersionNumber(parser);
	} else if (parser.meta.mode === "version") {
		throw parser.unexpected({
			description: descriptions.SEMVER.MISSING_PATCH_VERSION,
		});
	}

	if (parser.matchToken("Dot")) {
		throw parser.unexpected({
			description: descriptions.SEMVER.EXCESSIVE_VERSION_PARTS,
		});
	}

	// The dash is optional in loose mode. eg. 1.2.3pre
	let prerelease: VersionPrereleaseParts = [];
	if (
		parser.eatToken("Dash") ||
		(parser.options.loose && parser.matchToken("Word"))
	) {
		prerelease = parseVersionQualifierParts(parser);
	}

	let build: VersionPrereleaseParts = [];
	if (parser.eatToken("Plus")) {
		build = parseVersionQualifierParts(parser);
	}

	if (major !== undefined && minor !== undefined && patch !== undefined) {
		return {
			type: "AbsoluteVersion",
			loc: parser.finishLoc(startPos),
			major,
			minor,
			patch,
			prerelease,
			build,
		};
	} else {
		return {
			type: "WildcardVersion",
			loc: parser.finishLoc(startPos),
			major,
			minor,
			patch,
			prerelease,
			build,
		};
	}
}

function parseVersionQualifierParts(
	parser: SemverParser,
): VersionPrereleaseParts {
	const parts: VersionPrereleaseParts = [];
	do {
		parts.push(parseVersionQualifierPart(parser));
	} while (parser.eatToken("Dot") !== undefined);
	return parts;
}

function parseVersionQualifierPart(parser: SemverParser): string | number {
	const parts: Array<string | number> = [];

	do {
		const token = parser.getToken();

		if (token.type === "Number" || token.type === "Word") {
			parser.nextToken();
			parts.push(token.value);
		} else if (token.type === "Dash") {
			parser.nextToken();
			parts.push("-");
		} else if (parser.options.loose && token.type === "Star") {
			// https://github.com/romefrontend/rome/issues/295
			parser.nextToken();
			parts.push("*");
		} else {
			throw parser.unexpected({
				description: descriptions.SEMVER.INVALID_QUANTIFIER_PART,
			});
		}
	} while (
		parser.matchToken("Number") ||
		parser.matchToken("Word") ||
		parser.matchToken("Dash")
	);

	if (parts.length === 1 && typeof parts[0] === "number") {
		return parts[0];
	} else {
		return parts.join("");
	}
}

function isWildcardToken(
	parser: SemverParser,
	token: TokenValues<Tokens>,
): boolean {
	if (token.type === "Star") {
		return true;
	}

	if (token.type === "Word") {
		return token.value === "x" || token.value === "X";
	}

	return false;
}

function parseVersionNumber(parser: SemverParser): undefined | number {
	const token = parser.getToken();

	if (token.type === "Number") {
		parser.nextToken();
		return token.value;
	}

	if (isWildcardToken(parser, token)) {
		if (parser.meta.mode === "version") {
			throw parser.unexpected({
				description: descriptions.SEMVER.WILDCARD_IN_VERSION,
			});
		}

		parser.nextToken();
	} else {
		throw parser.unexpected({
			description: descriptions.SEMVER.INVALID_VERSION_NUMBER,
		});
	}

	return undefined;
}

function parseLogicalOr(parser: SemverParser, left: RangeNode): LogicalOrNode {
	parser.nextToken();
	eatSpaceToken(parser);

	const right = parseExpression(parser);
	return {
		loc: parser.finishLoc(parser.getLoc(left).start),
		type: "LogicalOr",
		left,
		right,
	};
}

function validateRangeSide(
	parser: SemverParser,
	node: RangeNode,
): VersionNode | WildcardNode {
	// In loose mode, we allow ranges to be a bare wildcard instead of a version
	// eg. * - 1.2.3
	if (node.type === "WildcardVersion" || node.type === "AbsoluteVersion") {
		return node;
	}

	if (node.type === "Wildcard" && parser.options.loose) {
		return node;
	}

	throw parser.unexpected({
		...descriptions.SEMVER.INVALID_RANGE,
		start: parser.getLoc(node).start,
	});
}

function parseVersionRange(
	parser: SemverParser,
	left: RangeNode,
): VersionRangeNode {
	parser.nextToken();
	eatSpaceToken(parser);

	const right = parseVersionOrWildcard(parser);

	return {
		type: "VersionRange",
		loc: parser.finishLoc(parser.getLoc(left).start),
		left: validateRangeSide(parser, left),
		right: validateRangeSide(parser, right),
	};
}

function parseWildcard(parser: SemverParser): WildcardNode {
	const startPos = parser.getPosition();
	parser.nextToken();
	return {type: "Wildcard", loc: parser.finishLoc(startPos)};
}

function parseAtomOperator(
	parser: SemverParser,
	token: Tokens["Operator"],
): ComparatorNode {
	const startPos = parser.getPosition();
	parser.nextToken();
	eatSpaceToken(parser);

	const version = parseVersionOrWildcard(parser);

	return {
		type: "Comparator",
		loc: parser.finishLoc(startPos),
		operator: token.value,
		version,
	};
}

function isVersionCharacter(
	parser: SemverParser,
	token: TokenValues<Tokens>,
): boolean {
	if (parser.options.loose && token.type === "Word") {
		return token.value === "v";
	}

	return false;
}

function parseAtomStartPipe(parser: SemverParser) {
	if (parser.options.loose) {
		// A bare pipe in an atom start position is treated the same as a wildcard...
		// Why...? Because node-semver allows it lol
		// > satisfies('1.2.3', '||') === true
		return parseWildcard(parser);
	} else {
		throw parser.unexpected({
			description: descriptions.SEMVER.BARE_PIPE_WITHOUT_LOOSE,
		});
	}
}

function parseAtomStartWord(parser: SemverParser, token: Tokens["Word"]) {
	if (isWildcardToken(parser, token)) {
		return parseWildcard(parser);
	} else if (isVersionCharacter(parser, token)) {
		return parseVersion(parser);
	} else {
		throw parser.unexpected({
			description: descriptions.SEMVER.UNEXPECTED_WORD(token.value),
		});
	}
}

function parseAtom(parser: SemverParser) {
	const token = parser.getToken();

	switch (token.type) {
		case "Number":
			return parseVersion(parser);

		case "Operator":
			return parseAtomOperator(parser, token);

		case "Star":
			return parseWildcard(parser);

		case "Pipe":
			return parseAtomStartPipe(parser);

		case "Word":
			return parseAtomStartWord(parser, token);

		default:
			throw parser.unexpected({
				description: descriptions.SEMVER.UNKNOWN_START,
			});
	}
}

function parseLogicalAnd(parser: SemverParser, left: RangeNode): LogicalAndNode {
	const right = parseExpression(parser);

	return {
		type: "LogicalAnd",
		left,
		right,
		loc: {
			filename: parser.filename,
			start: parser.getLoc(left).start,
			end: parser.getLoc(right).end,
		},
	};
}

function parseExpression(parser: SemverParser): RangeNode {
	const left = parseAtom(parser);
	eatSpaceToken(parser);

	if (parser.matchToken("RangeDash")) {
		return parseVersionRange(parser, left);
	}

	if (parser.matchToken("Pipe")) {
		return parseLogicalOr(parser, left);
	}

	if (!parser.matchToken("EOF")) {
		return parseLogicalAnd(parser, left);
	}

	return left;
}

function parseInitialRange(parser: SemverParser): RangeNode {
	// Allow spaces at the beginning, spaces at the end have been removed by the trimRight in the constructor
	eatSpaceToken(parser);

	// Empty string is an implicit wildcard in loose mode
	if (parser.matchToken("EOF") && parser.options.loose) {
		return parseWildcard(parser);
	}

	const expr = parseExpression(parser);
	parser.finalize();

	return expr;
}

function parseInitialVersion(parser: SemverParser): AbsoluteVersionNode {
	const node = parseInitialRange(parser);

	// Verify the return value in version mode
	if (node.type !== "AbsoluteVersion") {
		throw parser.unexpected({
			...descriptions.SEMVER.EXPECTED_VERSION,
			start: parser.getLoc(node).start,
		});
	}

	return node;
}

export function parseSemverRange(opts: SemverParserOptions): RangeNode {
	return parseInitialRange(createSemverParser(opts, {mode: "range"}));
}

export function parseSemverVersion(
	opts: SemverParserOptions,
): AbsoluteVersionNode {
	return parseInitialVersion(createSemverParser(opts, {mode: "version"}));
}
