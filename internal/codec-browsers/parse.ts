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
import {ZeroIndexed} from "@internal/numbers";

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

	tokenize(
		parser: ParserCore<BrowserQueryParserTypes>,
		index: ZeroIndexed,
	): TokenValues<BrowserQueryTokens> | undefined {
		const char = parser.getInputCharOnly(index);

		if (char === "-") {
			return parser.finishToken("Hyphen");
		}
		if (char === ">") {
			if (parser.getInputCharOnly(index.increment()) === "=") {
				return parser.finishToken("GE", index.add(2));
			}
			return parser.finishToken("GT");
		}
		if (char === "<") {
			if (parser.getInputCharOnly(index.increment()) === "=") {
				return parser.finishToken("LE", index.add(2));
			}
			return parser.finishToken("LT");
		}
		if (char === ",") {
			return parser.finishToken("Or");
		}

		const twoCharWord = parser.getInputRange(index, 2)[0].toLowerCase();
		if (twoCharWord === "or") {
			return parser.finishToken("Or", index.add(2));
		}
		if (twoCharWord === "in") {
			return parser.finishToken("In", index.add(2));
		}

		const threeCharWord = parser.getInputRange(index, 3)[0].toLowerCase();
		if (threeCharWord === "all") {
			return parser.finishToken("All", index.add(3));
		}
		if (threeCharWord === "day") {
			index = index.add(3);
			if (parser.getInputCharOnly(index) === "s") {
				index = index.increment();
			}
			return parser.finishToken("Days", index);
		}
		if (threeCharWord === "not") {
			return parser.finishToken("Not", index.add(3));
		}
		if (threeCharWord === "and") {
			return parser.finishToken("And", index.add(3));
		}

		const fourCharWord = parser.getInputRange(index, 4)[0].toLowerCase();
		if (fourCharWord === "year") {
			index = index.add(4);
			if (parser.getInputCharOnly(index) === "s") {
				index = index.increment();
			}
			return parser.finishToken("Years", index);
		}
		if (fourCharWord === "dead") {
			return parser.finishToken("Dead", index.add(4));
		}
		if (fourCharWord === "last") {
			return parser.finishToken("Last", index.add(4));
		}

		const fiveCharWord = parser.getInputRange(index, 5)[0].toLowerCase();
		if (fiveCharWord === "cover") {
			return parser.finishToken("Cover", index.add(5));
		}
		if (fiveCharWord === "since") {
			return parser.finishToken("Since", index.add(5));
		}
		if (fiveCharWord === "month") {
			index = index.add(5);
			if (parser.getInputCharOnly(index) === "s") {
				index = index.increment();
			}
			return parser.finishToken("Months", index);
		}

		if (parser.getInputRange(index, 6)[0].toLowerCase() === "modern") {
			return parser.finishToken("Modern", index.add(6));
		}

		const sevenCharWord = parser.getInputRange(index, 7)[0].toLowerCase();
		if (sevenCharWord === "default") {
			index = index.add(7);
			if (parser.getInputCharOnly(index) === "s") {
				index = index.increment();
			}
			return parser.finishToken("Modern", index.add(7));
		}
		if (sevenCharWord === "version") {
			index = index.add(7);
			if (parser.getInputCharOnly(index) === "s") {
				index = index.increment();
			}
			return parser.finishToken("Versions", index);
		}
		if (sevenCharWord === "current") {
			return parser.finishToken("Current", index.add(7));
		}

		const tenCharWord = parser.getInputRange(index, 10)[0].toLowerCase();
		if (tenCharWord === "maintained") {
			return parser.finishToken("Maintained", index.add(10));
		}
		if (tenCharWord === "unreleased") {
			return parser.finishToken("Unreleased", index.add(10));
		}

		if (parser.getInputRange(index, 13)[0].toLowerCase() === "major version") {
			index = index.add(13);
			if (parser.getInputCharOnly(index) === "s") {
				index = index.increment();
			}

			return parser.finishToken("MajorVersions", index);
		}

		if (isDigit(char) || char === ".") {
			const [value, endIndex] = parser.readInputFrom(
				index,
				(readChar) => isDigit(readChar) || readChar === ".",
			);

			if (parser.getInputCharOnly(endIndex) === "%") {
				return parser.finishValueToken(
					"Percentage",
					parseFloat(value),
					endIndex.increment(),
				);
			}

			return parser.finishValueToken("Number", parseFloat(value), endIndex);
		}

		if (isAlpha(char)) {
			const [value, endIndex] = parser.readInputFrom(
				index,
				(readChar) => isAlpha(readChar) || readChar === "_",
			);

			return parser.finishValueToken("String", value, endIndex);
		}

		return parser.finishValueToken("Invalid", char);
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
		if (newTarget != null) {
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
						target: targets.pop()!,
						and: newTarget,
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
