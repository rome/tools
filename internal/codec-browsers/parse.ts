import {
	BaseTokens,
	ParserCore,
	ParserOptions,
	SimpleToken,
	TokenValues,
	ValueToken,
	createParser,
	isAlpha,
	isDigit,
} from "@internal/parser-core";
import {
	AnyTargetBrowser,
	TargetBrowser,
	TargetBrowserCoverage,
	TargetBrowserLast,
	TargetBrowserRange,
	TargetBrowserRangeOperator,
	TargetBrowserSince,
	TargetBrowserState,
	TargetBrowserUsage,
	TargetOperator,
	TargetState,
	TargetUnit,
} from "@internal/codec-browsers/resolve";
import {descriptions} from "@internal/diagnostics";

type BrowserQueryTokens = BaseTokens & {
	String: ValueToken<"String", string>;
	Number: ValueToken<"Number", number>;
	Hyphen: SimpleToken<"Hyphen">;
	LT: SimpleToken<"LT">;
	GT: SimpleToken<"GT">;
	LE: SimpleToken<"LE">;
	GE: SimpleToken<"GE">;
	Or: SimpleToken<"Or">;
	And: SimpleToken<"And">;
	Not: SimpleToken<"Not">;
	Last: SimpleToken<"Last">;
	Dead: SimpleToken<"Dead">;
	Current: SimpleToken<"Current">;
	Maintained: SimpleToken<"Maintained">;
	Unreleased: SimpleToken<"Unreleased">;
	Percentage: ValueToken<"Percentage", number>;
	In: SimpleToken<"In">;
	Modern: SimpleToken<"Modern">;
	Cover: SimpleToken<"Cover">;
	Since: SimpleToken<"Since">;
	Versions: SimpleToken<"Versions">;
	MajorVersions: SimpleToken<"MajorVersions">;
	Years: SimpleToken<"Years">;
	Months: SimpleToken<"Months">;
	Days: SimpleToken<"Days">;
	All: SimpleToken<"All">;
};

type BrowserQueryParserTypes = {
	tokens: BrowserQueryTokens;
	state: {};
	options: ParserOptions;
	meta: void;
};

export const browserQueryParser = createParser<BrowserQueryParserTypes>({
	diagnosticLanguage: "browserquery",
	ignoreWhitespaceTokens: true,
	caseInsensitiveTokenMatches: true,

	tokenize(parser, tokenizer): TokenValues<BrowserQueryTokens> | undefined {
		if (tokenizer.consume("-")) {
			return tokenizer.finishToken("Hyphen");
		}

		if (tokenizer.consume(">")) {
			if (tokenizer.consume("=")) {
				return tokenizer.finishToken("GE");
			} else {
				return tokenizer.finishToken("GT");
			}
		}

		if (tokenizer.consume("<")) {
			if (tokenizer.consume("=")) {
				return tokenizer.finishToken("LE");
			} else {
				return tokenizer.finishToken("LT");
			}
		}

		if (tokenizer.consume(",")) {
			return tokenizer.finishToken("Or");
		}

		if (tokenizer.consume("or")) {
			return tokenizer.finishToken("Or");
		}

		if (tokenizer.consume("in")) {
			return tokenizer.finishToken("In");
		}

		if (tokenizer.consume("all")) {
			return tokenizer.finishToken("All");
		}

		if (tokenizer.consume("day")) {
			tokenizer.consume("s");
			return tokenizer.finishToken("Days");
		}

		if (tokenizer.consume("year")) {
			tokenizer.consume("s");
			return tokenizer.finishToken("Years");
		}

		if (tokenizer.consume("not")) {
			return tokenizer.finishToken("Not");
		}

		if (tokenizer.consume("and")) {
			return tokenizer.finishToken("And");
		}

		if (tokenizer.consume("dead")) {
			return tokenizer.finishToken("Dead");
		}

		if (tokenizer.consume("last")) {
			return tokenizer.finishToken("Last");
		}

		if (tokenizer.consume("cover")) {
			return tokenizer.finishToken("Cover");
		}

		if (tokenizer.consume("since")) {
			return tokenizer.finishToken("Since");
		}

		if (tokenizer.consume("month")) {
			tokenizer.consume("s");
			return tokenizer.finishToken("Months");
		}

		if (tokenizer.consume("modern")) {
			return tokenizer.finishToken("Modern");
		}

		if (tokenizer.consume("default")) {
			tokenizer.consume("s");
			return tokenizer.finishToken("Modern");
		}

		if (tokenizer.consume("version")) {
			tokenizer.consume("s");
			return tokenizer.finishToken("Versions");
		}

		if (tokenizer.consume("current")) {
			return tokenizer.finishToken("Current");
		}

		if (tokenizer.consume("maintained")) {
			return tokenizer.finishToken("Maintained");
		}

		if (tokenizer.consume("unreleased")) {
			return tokenizer.finishToken("Unreleased");
		}

		if (tokenizer.consume("major version")) {
			tokenizer.consume("s");
			return tokenizer.finishToken("MajorVersions");
		}

		const char = tokenizer.get();
		if (isDigit(char) || char === ".") {
			const value = tokenizer.read(
				(readChar) => isDigit(readChar) || readChar === ".",
			);

			if (tokenizer.consume("%")) {
				return tokenizer.finishValueToken(
					"Percentage",
					parseFloat(value),
				);
			}

			return tokenizer.finishValueToken("Number", parseFloat(value));
		}

		if (isAlpha(char)) {
			const value = tokenizer.read(
				(readChar) => isAlpha(readChar) || readChar === "_",
			);

			return tokenizer.finishValueToken("String", value);
		}

		return tokenizer.finishValueToken("Invalid", char);
	},
});

export function parseBrowserQuery(options: ParserOptions): AnyTargetBrowser[] {
	const targets: AnyTargetBrowser[] = [];

	let combination = false;
	let inverted = false;

	const parser = browserQueryParser.create(options);

	while (!parser.matchToken("EOF")) {
		let newTarget: AnyTargetBrowser | undefined = undefined;

		switch (parser.getToken().type) {
			case "String": {
				newTarget = parseString(parser);
				break;
			}

			case "Cover": {
				newTarget = parseCover(parser);
				break;
			}

			case "Since": {
				newTarget = parseSince(parser);
				break;
			}

			case "Last": {
				newTarget = parseLast(parser);
				break;
			}

			case "Maintained":
			case "Unreleased":
			case "Current": {
				newTarget = parseState(parser);
				break;
			}

			case "LT":
			case "GT":
			case "LE":
			case "GE": {
				newTarget = parseUsage(parser);
				break;
			}

			case "Modern":
			case "Dead": {
				newTarget = {
					type: "TargetBrowserPreset",
					preset: parser.getToken().type.toLowerCase() as "modern" | "dead",
				};
				parser.nextToken();
				break;
			}

			// Special elements
			case "And": {
				combination = true;
				parser.eatToken("And");
				break;
			}

			case "Not": {
				inverted = !inverted;
				parser.eatToken("Not");
				break;
			}

			// Don't care about them
			case "Or": {
				parser.eatToken("Or");
				break;
			}

			case "In":
			case "Number":
			case "Percentage":
			case "Hyphen":
			case "Versions":
			case "Years":
			case "Months":
			case "Days":
			case "Invalid": {
				throw parser.unexpected({
					description: descriptions.BROWSERQUERY.EXPECTED_NEW_QUERY,
					token: parser.getToken(),
				});
			}
		}

		// Invert / combine if required
		if (newTarget !== undefined) {
			if (inverted) {
				newTarget = {
					type: "TargetBrowserInversion",
					target: newTarget,
				};
			}

			if (combination) {
				if (targets.length === 0) {
					throw parser.unexpected({
						description: descriptions.BROWSERQUERY.AND_WITHOUT_QUERY,
						token: parser.getToken(),
					});
				} else {
					newTarget = {
						type: "TargetBrowserCombination",
						left: targets.pop()!,
						right: newTarget,
					};
				}
			}

			targets.push(newTarget);

			// Reset special attributes
			combination = false;
			inverted = false;
			newTarget = undefined;
		}
	}

	return targets;
}

function parseString(
	parser: ParserCore<BrowserQueryParserTypes>,
): TargetBrowser | TargetBrowserRange | TargetBrowserRangeOperator {
	const browser = parser.expectToken("String").value;

	switch (parser.getToken().type) {
		case "GT":
		case "LT":
		case "GE":
		case "LE": {
			const operator = parser.getToken().type as TargetOperator;
			parser.nextToken();

			return {
				type: "TargetBrowserRangeOperator",
				browser,
				version: parser.expectToken(
					"Number",
					descriptions.BROWSERQUERY.EXPECTED_VERSION,
				).value,
				operator,
			};
		}

		case "Number": {
			const version = parser.expectToken("Number").value;

			if (parser.getToken().type !== "Hyphen") {
				return {
					type: "TargetBrowser",
					browser,
					version,
				};
			}

			// Skip Hyphen
			parser.eatToken("Hyphen");

			return {
				type: "TargetBrowserRange",
				browser,
				version,
				to: parser.expectToken(
					"Number",
					descriptions.BROWSERQUERY.EXPECTED_VERSION,
				).value,
			};
		}

		case "All": {
			parser.eatToken("All");
			return {
				type: "TargetBrowser",
				browser,
				version: "all",
			};
		}

		default: {
			throw parser.unexpected({
				description: descriptions.BROWSERQUERY.EXPECTED_OPERATOR_OR_VERSION,
				token: parser.getToken(),
			});
		}
	}
}

function parseCover(
	parser: ParserCore<BrowserQueryParserTypes>,
): TargetBrowserCoverage {
	parser.eatToken("Cover");

	const coverage = parser.expectToken(
		"Percentage",
		descriptions.BROWSERQUERY.EXPECTED_PERCENTAGE,
	).value;

	// Optional region
	if (parser.getToken().type !== "In") {
		return {
			type: "TargetBrowserCoverage",
			coverage,
		};
	}

	// Skip In
	parser.eatToken("In");

	return {
		type: "TargetBrowserCoverage",
		coverage,
		region: parser.expectToken(
			"String",
			descriptions.BROWSERQUERY.EXPECTED_REGION,
		).value,
	};
}

function parseSince(
	parser: ParserCore<BrowserQueryParserTypes>,
): TargetBrowserSince {
	parser.eatToken("Since");

	let dateStr = parser.expectToken(
		"Number",
		descriptions.BROWSERQUERY.EXPECTED_DATE,
	).value.toString();

	for (let i = 0; i < 2; i++) {
		if (parser.getToken().type !== "Hyphen") {
			dateStr += "-01";
			continue;
		}

		// Skip Hyphen
		parser.eatToken("Hyphen");

		const num = parser.expectToken(
			"Number",
			descriptions.BROWSERQUERY.EXPECTED_DATE,
		).value.toString();

		dateStr += `-${num.length === 1 ? `0${num}` : num}`;
	}

	return {
		type: "TargetBrowserSince",
		since: new Date(`${dateStr}T00:00:00.000Z`).getTime(), // As number for serialization, T0...0Z to use UTC
	};
}

function parseLast(
	parser: ParserCore<BrowserQueryParserTypes>,
): TargetBrowserLast {
	parser.eatToken("Last");

	const qty = parser.expectToken(
		"Number",
		descriptions.BROWSERQUERY.EXPECTED_NUMBER,
	).value;
	let browser: string | undefined = undefined;

	// Browser is optional
	if (parser.getToken().type === "String") {
		browser = parser.expectToken("String").value;
	}

	if (
		!["Years", "Months", "Days", "Versions", "MajorVersions"].includes(
			parser.getToken().type,
		)
	) {
		throw parser.unexpected({
			description: descriptions.BROWSERQUERY.EXPECTED_UNIT,
			token: parser.getToken(),
		});
	}

	// Because there are multiple token type using eatToken or expectToken is not feasible
	// Instead move and get the previous one
	parser.nextToken();
	return {
		type: "TargetBrowserLast",
		qty,
		unit: parser.getPreviousToken().type.toLowerCase() as TargetUnit,
		browser,
	};
}

function parseState(
	parser: ParserCore<BrowserQueryParserTypes>,
): TargetBrowserState {
	const state = parser.getToken().type.toLowerCase() as TargetState;
	parser.nextToken();

	// Browser is optional
	if (parser.getToken().type !== "String") {
		const target: TargetBrowserState = {
			type: "TargetBrowserState",
			state,
		};

		// Ignore Versions keyword
		parser.eatToken("Versions");

		return target;
	}

	const target: TargetBrowserState = {
		type: "TargetBrowserState",
		browser: parser.expectToken("String").value,
		state,
	};

	// Ignore Versions keyword
	parser.eatToken("Versions");

	return target;
}

function parseUsage(
	parser: ParserCore<BrowserQueryParserTypes>,
): TargetBrowserUsage {
	const operator = parser.getToken().type as TargetOperator;
	parser.nextToken();

	const usage = parser.expectToken(
		"Percentage",
		descriptions.BROWSERQUERY.EXPECTED_PERCENTAGE,
	).value;

	// If there's no region
	if (parser.getToken().type !== "In") {
		return {
			type: "TargetBrowserUsage",
			usage,
			operator,
		};
	}

	// Skip In
	parser.eatToken("In");

	return {
		type: "TargetBrowserUsage",
		usage,
		operator,
		region: parser.expectToken(
			"String",
			descriptions.BROWSERQUERY.EXPECTED_REGION,
		).value,
	};
}
