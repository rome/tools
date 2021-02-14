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
	Modern: SimpleToken<"Modern">
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

function parseBrowserQuery(options: ParserOptions) {
	const targets: AnyTargetBrowser[] = [];

	const parser = browserQueryParser.create(options);

	while (!parser.matchToken("EOF")) {
		switch (parser.getToken().type) {
			case "String":
				const name = (parser.getToken() as BrowserQueryTokens["String"]).value;

				parser.nextToken();
				// if (parser.getToken().type !== "GT" || parser.getToken().type !== "") {
				// 	parser.unexpectedDiagnostic({
				// 		description: {message: markup`Expected a `},
				// 	});
				// }

				break;
			case "Maintained":
				break;
			case "Or":
				parser.nextToken();
				continue;
			case "Dead":
				break;
			case "Percentage":
				break;
			case "In":
				break;
			case "Hyphen":
				break;
			case "LT":
				break;
			case "GT":
				break;
			case "Modern":
				break;
			case "Not":
				break;
			case "Last":
				break;
			case "Number":
				break;
			case "And":
				break;
			case "Unreleased":
				break;
			case "LE":
				break;
			case "Current":
				break;
			case "GE":
				break;

			case "Invalid": {
				parser.unexpectedDiagnostic({
					description: {message: markup`Invalid token`},
				});
				break;
			}
		}
		parser.nextToken();
	}
}
