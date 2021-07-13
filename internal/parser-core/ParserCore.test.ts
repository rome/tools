import {test} from "rome";
import {
	BaseTokens,
	ParserCore,
	ParserOptions,
	SimpleToken,
	TokenValues,
	ValueToken,
	createParser,
	isDigit,
} from "@internal/parser-core/index";
import {dedent} from "@internal/string-utils";
import {markup} from "@internal/markup";
import {isNewline} from "@internal/css-parser/utils";
import {createRelativePath} from "@internal/path";

type TestTokens = BaseTokens & {
	Comment: ValueToken<"Comment", string>;
	String: ValueToken<"String", string>;
	Number: ValueToken<"Number", number>;
	NewLine: SimpleToken<"NewLine">;
};

type TestParserTypes = {
	tokens: TestTokens;
	state: {};
	options: Omit<ParserOptions, "ignoreWhitespaceTokens">;
	meta: void;
};

test(
	"test parsing",
	(t) => {
		// Simple testing parser
		const testParser = createParser<TestParserTypes>({
			diagnosticLanguage: "unknown",
			ignoreWhitespaceTokens: true,

			tokenize(
				parser,
				tokenizer,
			): TokenValues<TestParserTypes["tokens"]> | undefined {
				const char = tokenizer.get();

				if (tokenizer.consume("//")) {
					let value = "";

					while (!(isNewline(tokenizer.get()) || tokenizer.isEOF())) {
						value += tokenizer.take(1);
					}

					return tokenizer.finishValueToken("Comment", value);
				}

				if (tokenizer.consume('"')) {
					let value = "";

					while (!tokenizer.startsWith('"')) {
						if (tokenizer.isEOF()) {
							parser.unexpectedDiagnostic({
								description: {message: markup`Unterminated string`},
							});
							break;
						}

						value += tokenizer.take(1);
					}

					tokenizer.assert('"');

					return tokenizer.finishValueToken("String", value);
				}

				if (isDigit(char)) {
					let value = "";

					while (
						isDigit(tokenizer.get()) &&
						!isNewline(tokenizer.get()) &&
						!tokenizer.isEOF()
					) {
						value += tokenizer.take(1);
					}

					return tokenizer.finishValueToken("Number", parseInt(value));
				}

				return tokenizer.finishValueToken("Invalid", char);
			},
		});

		// Testing helper
		function runParserTests(parser: ParserCore<TestParserTypes>): object {
			const parsed: object[] = [];

			while (!parser.matchToken("EOF")) {
				const loc = parser.finishLocFromToken(parser.getToken());

				parsed.push({
					loc,
					snapshot: {
						nextTokenIndex: parser.save().nextTokenIndex,
						currentToken: parser.save().currentToken,
						prevToken: parser.save().prevToken,
					},
				});

				switch (parser.getToken().type) {
					case "Comment": {
						parser.registerComment(
							parser.comments.createComment({
								type: "CommentLine",
								loc,
								value: (parser.getToken() as TestTokens["Comment"]).value,
							}),
						);
						break;
					}
					case "Invalid": {
						parser.unexpectedDiagnostic({
							description: {message: markup`Invalid token`},
						});
						break;
					}
				}
				parser.nextToken();
			}

			return {
				parsed,
				corrupt: parser.state.corrupt,
				diagnostics: parser.getDiagnostics(),
				path: parser.path,
				comments: parser.state.comments,
			};
		}

		const parser = testParser.create({
			input: dedent`
				"im a string"

				4

				// Comment
			`,
			path: createRelativePath("0.test"),
		});

		const parser1 = testParser.create({
			path: createRelativePath("1.test"),
			input: "a", // Invalid
		});

		const parser2 = testParser.create({
			path: createRelativePath("2.test"),
			input: `"i'm an unterminated string`,
		});

		t.snapshot(runParserTests(parser));
		t.snapshot(runParserTests(parser1));
		t.snapshot(runParserTests(parser2));
	},
);
