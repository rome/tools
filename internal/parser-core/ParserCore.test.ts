import {test} from "rome";
import {
	BaseTokens,
	ParserOptions,
	SimpleToken,
	ValueToken,
	createParser,
	isDigit,
} from "@internal/parser-core/index";
import {Number0, ob1Add, ob1Inc} from "@internal/ob1";
import {createUnknownFilePath} from "@internal/path";
import {dedent} from "@internal/string-utils";
import {markup} from "@internal/markup";
import {isNewline} from "@internal/css-parser/utils";

type TestTokens = BaseTokens & {
	Comment: ValueToken<"Comment", string>;
	String: ValueToken<"String", string>;
	Number: ValueToken<"Number", number>;
	NewLine: SimpleToken<"NewLine">;
};

test(
	"test parsing",
	(t) => {

		// Simple testing parser
		const createTestParser = createParser((ParserCore) => {
			class TestParser extends ParserCore<TestTokens> {
				constructor(opts: ParserOptions) {
					// Random category
					super(opts, "tests/failure", {});
					this.ignoreWhitespaceTokens = true;
				}

				protected tokenize(index: Number0): TestTokens[keyof TestTokens] {
					const char = this.getInputCharOnly(index);

					if (char === "/" && this.getInputCharOnly(index, 1) === "/") {
						index = ob1Add(index, 2);
						let value = "";

						while (
							!isNewline(this.getInputCharOnly(index)) &&
							!this.isEOF(index)
						) {
							value += this.getInputCharOnly(index);
							index = ob1Add(index, 1);
						}

						return this.finishValueToken("Comment", value, index);
					}

					if (char === '"') {
						index = ob1Add(index, 1);
						let value = "";

						while (this.getInputCharOnly(index) !== '"') {
							if (this.isEOF(index)) {
								this.unexpectedDiagnostic({
									description: {message: markup`Unterminated string`},
								});
								break;
							}

							value += this.getInputCharOnly(index);
							index = ob1Add(index, 1);
						}

						return this.finishValueToken("String", value, ob1Inc(index));
					}

					if (isDigit(char)) {
						let value = "";

						while (
							isDigit(this.getInputCharOnly(index)) &&
							!isNewline(this.getInputCharOnly(index)) &&
							!this.isEOF(index)
						) {
							value += this.getInputCharOnly(index);
							index = ob1Add(index, 1);
						}

						return this.finishValueToken("Number", parseInt(value), index);
					}

					return this.finishValueToken("Invalid", char);
				}

				public test(): object {
					const parsed: Array<object> = [];

					while (!this.matchToken("EOF")) {
						const loc = this.finishLocFromToken(this.getToken());

						parsed.push({
							loc,
							snapshot: {
								nextTokenIndex: this.save().nextTokenIndex,
								currentToken: this.save().currentToken,
								prevToken: this.save().prevToken,
							},
						});

						switch (this.getToken().type) {
							case "Comment": {
								this.registerComment(
									this.comments.createComment({
										type: "CommentLine",
										loc,
										value: (this.getToken() as TestTokens["Comment"]).value,
									}),
								);
								break;
							}
							case "Invalid": {
								this.unexpectedDiagnostic({
									description: {message: markup`Invalid token`},
								});
								break;
							}
						}
						this.nextToken();
					}

					return {
						parsed,
						corrupt: this.state.corrupt,
						mtime: this.mtime,
						diagnostics: this.getDiagnostics(),
						filename: this.getFilenameAssert(),
						comments: this.state.comments,
					};
				}
			}

			return TestParser;
		});

		const parser = createTestParser({
			input: dedent`
				"im a string"

				4

				// Comment
			`,
			mtime: 78_235_964,
			path: createUnknownFilePath("0.test"),
			retainCarriageReturn: false,
		});

		const parser1 = createTestParser({
			path: createUnknownFilePath("1.test"),
			input: "a", // Invalid
		});

		const parser2 = createTestParser({
			path: createUnknownFilePath("2.test"),
			input: `"i'm an unterminated string`,
		});

		t.snapshot(parser.test());
		t.snapshot(parser1.test());
		t.snapshot(parser2.test());
	},
);
