import {
	BaseTokens,
	createParser, isAlpha, isDigit,
	ParserCore,
	ParserOptions,
	SimpleToken,
	TokenValues,
	ValueToken
} from "@internal/parser-core";
import {Number0, ob1Add, ob1Inc} from "@internal/ob1";
import {markup} from "@internal/markup";
import {AnyTargetBrowser} from "@internal/codec-browsers/resolve";
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
	VersionsKeyword: SimpleToken<"VersionsKeyword">
};

type BrowserQueryParserTypes = {
	tokens: BrowserQueryTokens;
	state: {};
	options: ParserOptions;
	meta: void;
};

const browserQueryParser = createParser<BrowserQueryParserTypes>({
	diagnosticLanguage: "browserquery",
	ignoreWhitespaceTokens: true,

	tokenize(parser: ParserCore<BrowserQueryParserTypes>, index: Number0): TokenValues<BrowserQueryTokens> | undefined {
		const char = parser.getInputCharOnly(index);

		if (parser.getInputRange(index, 8)[0] === "versions") {
			return parser.finishToken("VersionsKeyword", ob1Add(index, 8));
		}

		if (parser.getInputRange(index, 5)[0] === "cover") {
			return parser.finishToken("Cover", ob1Add(index, 5));
		}

		if (parser.getInputRange(index, 5)[0] === "since") {
			return parser.finishToken("Since", ob1Add(index, 5));
		}

		if (parser.getInputRange(index, 6)[0] === "modern") {
			return parser.finishToken("Modern", ob1Add(index, 6));
		}

		if (parser.getInputRange(index, 8)[0] === "defaults") {
			return parser.finishToken("Modern", ob1Add(index, 8));
		}

		if (parser.getInputRange(index, 7)[0] === "default") {
			return parser.finishToken("Modern", ob1Add(index, 7));
		}

		if (char === "-") {
			return parser.finishToken("Hyphen");
		}

		if (parser.getInputRange(index, 2)[0] === ">=") {
			return parser.finishToken("GE", ob1Add(index, 2));
		}

		if (parser.getInputRange(index, 2)[0] === "<=") {
			return parser.finishToken("LE", ob1Add(index, 2));
		}

		if (char === "<") {
			return parser.finishToken("GT");
		}

		if (char === ">") {
			return parser.finishToken("GT");
		}

		if (parser.getInputRange(index, 4)[0] === "dead") {
			return parser.finishToken("Dead", ob1Add(index, 4));
		}

		if (parser.getInputRange(index, 7)[0] === "current") {
			return parser.finishToken("Current", ob1Add(index, 7));
		}

		if (parser.getInputRange(index, 4)[0] === "last") {
			return parser.finishToken("Last", ob1Add(index, 4));
		}

		if (parser.getInputRange(index, 10)[0] === "maintained") {
			return parser.finishToken("Maintained", ob1Add(index, 10));
		}

		if (parser.getInputRange(index, 10)[0] === "unreleased") {
			return parser.finishToken("Unreleased", ob1Add(index, 10));
		}

		if (parser.getInputRange(index, 3)[0] === "not") {
			return parser.finishToken("Not", ob1Add(index, 3));
		}

		if (parser.getInputRange(index, 3)[0] === "and") {
			return parser.finishToken("And", ob1Add(index, 3));
		}

		if (parser.getInputRange(index, 2)[0] === "or") {
			return parser.finishToken("Or", ob1Add(index, 2));
		}
		if (char === ",") {
			return parser.finishToken("Or");
		}

		if (parser.getInputRange(index, 2)[0] === "in") {
			return parser.finishToken("In", ob1Add(index, 2));
		}

		if (isDigit(char) || char === ".") {
			let value = "";

			while (
				(isDigit(parser.getInputCharOnly(index)) || parser.getInputCharOnly(index) === ".") && !parser.isEOF(index)
				) {
				value += parser.getInputCharOnly(index);
				index = ob1Inc(index);
			}

			if (parser.getInputCharOnly(index, 1) === "%") {
				return parser.finishValueToken("Percentage", parseInt(value), ob1Inc(index));
			}

			return parser.finishValueToken("Number", parseInt(value), index);
		}

		if (isAlpha(char)) {
			index = ob1Inc(index);
			let value = "";

			while ((isAlpha(parser.getInputCharOnly(index)) || parser.getInputCharOnly(index) === "-") && !parser.isEOF(index)) {
				value += parser.getInputCharOnly(index);
				index = ob1Inc(index);
			}

			return parser.finishValueToken("String", value, ob1Inc(index));
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
		let newTarget: AnyTargetBrowser|undefined = undefined;

		switch (parser.getToken().type) {
			case "String":
				const browser = (parser.getToken() as BrowserQueryTokens["String"]).value;

				parser.nextToken();

				switch (parser.getToken().type) {
					case "GT":
					case "LT":
					case "GE":
					case "LE":
						parser.nextToken();

						if (parser.getToken().type !== "Number") {
							parser.unexpectedDiagnostic({
								description: descriptions.BROWSERQUERY.EXPECTED_VERSION,
								loc: parser.finishLocFromToken(parser.getToken())
							});
							break;
						}

						newTarget = {
							type: "TargetBrowserRangeOperator",
							browser,
							version: (parser.getToken() as BrowserQueryTokens["Number"]).value,
							operator: parser.getPreviousToken().type as "GT" | "LT" | "GE" | "LE"
						};
						break;

					case "Number":
						const version = (parser.getToken() as BrowserQueryTokens["Number"]).value;

						if (parser.lookaheadToken().type !== "Hyphen") {
							newTarget = {
								type: "TargetBrowser",
								browser,
								version,
							};
							break;
						}

						// Skip Hyphen
						parser.nextToken();
						parser.nextToken();

						if (parser.lookaheadToken().type !== "Number") {
							parser.unexpectedDiagnostic({
								description: descriptions.BROWSERQUERY.EXPECTED_VERSION,
								loc: parser.finishLocFromToken(parser.getToken())
							});
							break;
						}

						newTarget = {
							type: "TargetBrowserRange",
							browser,
							version,
							to: (parser.getToken() as BrowserQueryTokens["Number"]).value,
						};
						break;

					default:
						parser.unexpectedDiagnostic({
							description: descriptions.BROWSERQUERY.EXPECTED_OPERATOR_OR_VERSION,
							loc: parser.finishLocFromToken(parser.getToken())
						});
						break;
				}

				break;

			case "Modern":
			case "Dead":
				newTarget = {
					type: "TargetBrowserPreset",
					preset: parser.getToken().type.toLowerCase() as "modern" | "dead"
				}
				break;

			case "Cover":
				break;
			case "Since":
				break;
			case "Last":
				break;

			case "Maintained":
			case "Unreleased":
			case "Current":
				if (parser.lookaheadToken().type !== "String") {
					newTarget = {
						type: "TargetBrowserState",
						state: parser.getToken().type.toLowerCase() as "current" | "unreleased" | "maintained"
					}
					break;
				}

				parser.nextToken();

				newTarget = {
					type: "TargetBrowserState",
					browser: (parser.getToken() as BrowserQueryTokens["String"]).value,
					state: parser.getToken().type.toLowerCase() as "current" | "unreleased" | "maintained"
				}
				break;

			case "LT":
			case "GT":
			case "LE":
			case "GE":
				const operator = parser.getToken().type as "LT" | "GT" | "LE" | "GE";

				parser.nextToken();

				if (parser.getToken().type !== "Percentage") {
					parser.unexpectedDiagnostic({
						description: descriptions.BROWSERQUERY.EXPECTED_PERCENTAGE,
						loc: parser.finishLocFromToken(parser.getToken())
					});
					break;
				}

				const coverage = (parser.getToken() as BrowserQueryTokens["Percentage"]).value;

				if (parser.lookaheadToken().type !== "In") {
					newTarget = {
						type: "TargetBrowserCoverage",
						coverage,
						operator
					}
				}

				// Skip In
				parser.nextToken();
				parser.nextToken();

				if (parser.getToken().type !== "String") {
					parser.unexpectedDiagnostic({
						description: descriptions.BROWSERQUERY.EXPECTED_REGION,
						loc: parser.finishLocFromToken(parser.getToken())
					});
					break;
				}

				newTarget = {
					type: "TargetBrowserCoverage",
					coverage,
					operator,
					region: (parser.getToken() as BrowserQueryTokens["String"]).value
				}

				break;

			case "And":
				combination = true;
				break;

			case "Not":
				inverted = !inverted;
				break;

			// Don't care about them
			case "Or":
			case "VersionsKeyword":
				break;

			case "In":
			case "Number":
			case "Percentage":
			case "Hyphen":
				parser.unexpectedDiagnostic({
					description: descriptions.BROWSERQUERY.EXPECTED_NEW_QUERY,
					loc: parser.finishLocFromToken(parser.getToken())
				});
				break;

			case "Invalid": {
				parser.unexpectedDiagnostic({
					description: {message: markup`Invalid token`},
					loc: parser.finishLocFromToken(parser.getToken())
				});
				break;
			}
		}
		if (newTarget != null) {
			if (inverted) {
				newTarget = {
					type: "TargetBrowserInversion",
					target: newTarget,
				}
			}

			if (combination) {
				if (targets.length === 0) {
					parser.unexpectedDiagnostic({
						description: descriptions.BROWSERQUERY.AND_WITHOUT_TARGET,
						loc: parser.finishLocFromToken(parser.getToken())
					});
					break;
				}

				newTarget = {
					type: "TargetBrowserCombination",
					target: targets.pop()!,
					and: newTarget
				}
			}

			targets.push(newTarget);

			// Reset special attributes
			combination = false;
			inverted = false;
		}

		parser.nextToken();
	}

	return targets;
}
