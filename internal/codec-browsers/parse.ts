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
		index: Number0,
	): TokenValues<BrowserQueryTokens> | undefined {
		const char = parser.getInputCharOnly(index);

		if (parser.getInputRange(index, 14)[0].toLowerCase() === "major versions") {
			return parser.finishToken("MajorVersions", ob1Add(index, 14));
		}

		if (parser.getInputRange(index, 13)[0].toLowerCase() === "major version") {
			return parser.finishToken("MajorVersions", ob1Add(index, 13));
		}

		if (parser.getInputRange(index, 8)[0].toLowerCase() === "versions") {
			return parser.finishToken("Versions", ob1Add(index, 8));
		}

		if (parser.getInputRange(index, 3)[0].toLowerCase() === "all") {
			return parser.finishToken("All", ob1Add(index, 3));
		}

		if (parser.getInputRange(index, 5)[0].toLowerCase() === "cover") {
			return parser.finishToken("Cover", ob1Add(index, 5));
		}

		if (parser.getInputRange(index, 5)[0].toLowerCase() === "since") {
			return parser.finishToken("Since", ob1Add(index, 5));
		}

		if (parser.getInputRange(index, 6)[0].toLowerCase() === "modern") {
			return parser.finishToken("Modern", ob1Add(index, 6));
		}

		if (parser.getInputRange(index, 8)[0].toLowerCase() === "defaults") {
			return parser.finishToken("Modern", ob1Add(index, 8));
		}

		if (parser.getInputRange(index, 7)[0].toLowerCase() === "default") {
			return parser.finishToken("Modern", ob1Add(index, 7));
		}

		if (parser.getInputRange(index, 5)[0].toLowerCase() === "years") {
			return parser.finishToken("Years", ob1Add(index, 5));
		}

		if (parser.getInputRange(index, 4)[0].toLowerCase() === "year") {
			return parser.finishToken("Years", ob1Add(index, 4));
		}

		if (parser.getInputRange(index, 6)[0].toLowerCase() === "months") {
			return parser.finishToken("Months", ob1Add(index, 6));
		}

		if (parser.getInputRange(index, 5)[0].toLowerCase() === "month") {
			return parser.finishToken("Months", ob1Add(index, 5));
		}

		if (parser.getInputRange(index, 4)[0].toLowerCase() === "days") {
			return parser.finishToken("Days", ob1Add(index, 4));
		}

		if (parser.getInputRange(index, 3)[0].toLowerCase() === "day") {
			return parser.finishToken("Days", ob1Add(index, 3));
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
			return parser.finishToken("LT");
		}

		if (char === ">") {
			return parser.finishToken("GT");
		}

		if (parser.getInputRange(index, 4)[0].toLowerCase() === "dead") {
			return parser.finishToken("Dead", ob1Add(index, 4));
		}

		if (parser.getInputRange(index, 7)[0].toLowerCase() === "current") {
			return parser.finishToken("Current", ob1Add(index, 7));
		}

		if (parser.getInputRange(index, 4)[0].toLowerCase() === "last") {
			return parser.finishToken("Last", ob1Add(index, 4));
		}

		if (parser.getInputRange(index, 10)[0].toLowerCase() === "maintained") {
			return parser.finishToken("Maintained", ob1Add(index, 10));
		}

		if (parser.getInputRange(index, 10)[0].toLowerCase() === "unreleased") {
			return parser.finishToken("Unreleased", ob1Add(index, 10));
		}

		if (parser.getInputRange(index, 3)[0].toLowerCase() === "not") {
			return parser.finishToken("Not", ob1Add(index, 3));
		}

		if (parser.getInputRange(index, 3)[0].toLowerCase() === "and") {
			return parser.finishToken("And", ob1Add(index, 3));
		}

		if (parser.getInputRange(index, 2)[0].toLowerCase() === "or") {
			return parser.finishToken("Or", ob1Add(index, 2));
		}
		if (char === ",") {
			return parser.finishToken("Or");
		}

		if (parser.getInputRange(index, 2)[0].toLowerCase() === "in") {
			return parser.finishToken("In", ob1Add(index, 2));
		}

		if (isDigit(char) || char === ".") {
			let value = "";

			while (
				(isDigit(parser.getInputCharOnly(index)) ||
				parser.getInputCharOnly(index) === ".") &&
				!parser.isEOF(index)
			) {
				value += parser.getInputCharOnly(index);
				index = ob1Inc(index);
			}

			if (parser.getInputCharOnly(index) === "%") {
				return parser.finishValueToken(
					"Percentage",
					parseFloat(value),
					ob1Inc(index),
				);
			}

			return parser.finishValueToken("Number", parseFloat(value), index);
		}

		if (isAlpha(char)) {
			let value = "";

			while (
				(isAlpha(parser.getInputCharOnly(index)) ||
				parser.getInputCharOnly(index) === "_") &&
				!parser.isEOF(index)
			) {
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
		let newTarget: AnyTargetBrowser | undefined = undefined;

		switch (parser.getToken().type) {
			case "String": {
				const browser = (parser.getToken() as BrowserQueryTokens["String"]).value;

				parser.nextToken();

				switch (parser.getToken().type) {
					case "GT":
					case "LT":
					case "GE":
					case "LE": {
						parser.nextToken();

						if (parser.getToken().type !== "Number") {
							throw parser.unexpected({
								description: descriptions.BROWSERQUERY.EXPECTED_VERSION,
								token: parser.getToken(),
							});
						}

						newTarget = {
							type: "TargetBrowserRangeOperator",
							browser,
							version: (parser.getToken() as BrowserQueryTokens["Number"]).value,
							operator: parser.getPreviousToken().type as
								| "GT"
								| "LT"
								| "GE"
								| "LE",
						};
						break;
					}

					case "Number": {
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

						if (parser.getToken().type !== "Number") {
							throw parser.unexpected({
								description: descriptions.BROWSERQUERY.EXPECTED_VERSION,
								token: parser.getToken(),
							});
						}

						newTarget = {
							type: "TargetBrowserRange",
							browser,
							version,
							to: (parser.getToken() as BrowserQueryTokens["Number"]).value,
						};
						break;
					}

					case "All": {
						newTarget = {
							type: "TargetBrowser",
							browser,
							version: "all",
						};
						break;
					}

					default: {
						throw parser.unexpected({
							description: descriptions.BROWSERQUERY.EXPECTED_OPERATOR_OR_VERSION,
							token: parser.getToken(),
						});
					}
				}
				break;
			}

			case "Modern":
			case "Dead": {
				newTarget = {
					type: "TargetBrowserPreset",
					preset: parser.getToken().type.toLowerCase() as "modern" | "dead",
				};
				break;
			}

			case "Cover": {
				parser.nextToken();

				if (parser.getToken().type !== "Percentage") {
					throw parser.unexpected({
						description: descriptions.BROWSERQUERY.EXPECTED_PERCENTAGE,
						token: parser.getToken(),
					});
				}

				const coverage = (parser.getToken() as BrowserQueryTokens["Percentage"]).value;

				if (parser.lookaheadToken().type !== "In") {
					newTarget = {
						type: "TargetBrowserCoverage",
						coverage,
					};
					break;
				}

				// Skip In
				parser.nextToken();
				parser.nextToken();

				if (parser.getToken().type !== "String") {
					throw parser.unexpected({
						description: descriptions.BROWSERQUERY.EXPECTED_REGION,
						token: parser.getToken(),
					});
				}

				newTarget = {
					type: "TargetBrowserCoverage",
					coverage,
					region: (parser.getToken() as BrowserQueryTokens["String"]).value,
				};
				break;
			}

			case "Since": {
				parser.nextToken();

				if (parser.getToken().type !== "Number") {
					throw parser.unexpected({
						description: descriptions.BROWSERQUERY.EXPECTED_DATE,
						token: parser.getToken(),
					});
				}

				let dateStr = (parser.getToken() as BrowserQueryTokens["Number"]).value.toString();

				for (let i = 0; i < 2; i++) {
					if (parser.lookaheadToken().type !== "Hyphen") {
						break;
					}

					// Skip Hyphen
					parser.nextToken();
					parser.nextToken();

					if (parser.getToken().type !== "Number") {
						throw parser.unexpected({
							description: descriptions.BROWSERQUERY.EXPECTED_DATE,
							token: parser.getToken(),
						});
					}

					dateStr += `-${(parser.getToken() as BrowserQueryTokens["Number"]).value.toString()}`;
				}

				newTarget = {
					type: "TargetBrowserSince",
					since: new Date(dateStr).getTime(), // As number for serialization
				};

				break;
			}
			case "Last": {
				parser.nextToken();

				if (parser.getToken().type !== "Number") {
					throw parser.unexpected({
						description: descriptions.BROWSERQUERY.EXPECTED_NUMBER,
						token: parser.getToken(),
					});
				}

				const qty = (parser.getToken() as BrowserQueryTokens["Number"]).value;
				let browser: string | undefined = undefined;

				parser.nextToken();

				if (parser.getToken().type === "String") {
					browser = (parser.getToken() as BrowserQueryTokens["String"]).value;

					parser.nextToken();
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

				newTarget = {
					type: "TargetBrowserLast",
					qty,
					unit: parser.getToken().type.toLowerCase() as
						| "years"
						| "months"
						| "days"
						| "versions"
						| "majorversions",
					browser,
				};

				break;
			}

			case "Maintained":
			case "Unreleased":
			case "Current": {
				if (parser.lookaheadToken().type !== "String") {
					newTarget = {
						type: "TargetBrowserState",
						state: parser.getToken().type.toLowerCase() as
							| "current"
							| "unreleased"
							| "maintained",
					};

					if (parser.lookaheadToken().type === "Versions") {
						// Ignore Versions keyword
						parser.nextToken();
					}
					break;
				}

				parser.nextToken();

				newTarget = {
					type: "TargetBrowserState",
					browser: (parser.getToken() as BrowserQueryTokens["String"]).value,
					state: parser.getPreviousToken().type.toLowerCase() as
						| "current"
						| "unreleased"
						| "maintained",
				};

				if (parser.lookaheadToken().type === "Versions") {
					// Ignore Versions keyword
					parser.nextToken();
				}
				break;
			}

			case "LT":
			case "GT":
			case "LE":
			case "GE": {
				const operator = parser.getToken().type as "LT" | "GT" | "LE" | "GE";

				parser.nextToken();

				if (parser.getToken().type !== "Percentage") {
					throw parser.unexpected({
						description: descriptions.BROWSERQUERY.EXPECTED_PERCENTAGE,
						token: parser.getToken(),
					});
				}

				const usage = (parser.getToken() as BrowserQueryTokens["Percentage"]).value;

				if (parser.lookaheadToken().type !== "In") {
					newTarget = {
						type: "TargetBrowserUsage",
						usage,
						operator,
					};
					break;
				}

				// Skip In
				parser.nextToken();
				parser.nextToken();

				if (parser.getToken().type !== "String") {
					throw parser.unexpected({
						description: descriptions.BROWSERQUERY.EXPECTED_REGION,
						token: parser.getToken(),
					});
				}

				newTarget = {
					type: "TargetBrowserUsage",
					usage,
					operator,
					region: (parser.getToken() as BrowserQueryTokens["String"]).value,
				};

				break;
			}

			case "And": {
				combination = true;
				break;
			}

			case "Not": {
				inverted = !inverted;
				break;
			}

			// Don't care about them
			case "Or":
				break;

			case "In":
			case "Number":
			case "Percentage":
			case "Hyphen":
			case "Versions":
			case "Years":
			case "Months":
			case "Days": {
				throw parser.unexpected({
					description: descriptions.BROWSERQUERY.EXPECTED_NEW_QUERY,
					token: parser.getToken(),
				});
			}

			case "Invalid": {
				throw parser.unexpected({
					description: {message: markup`Invalid token`},
					token: parser.getToken(),
				});
			}
		}
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

		parser.nextToken();
	}

	return targets;
}
