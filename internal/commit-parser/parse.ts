import {CommitRoot} from "@romefrontend/ast";
import {descriptions} from "@romefrontend/diagnostics";
import {Number0, ob1Add} from "@romefrontend/ob1";
import {
	ParserOptions,
	TokenValues,
	createParser,
} from "@romefrontend/parser-core";

import {Symbols, Tokens} from "./types";

export const createCommitParser = createParser((ParserCore) =>
	class CommitParser extends ParserCore<Tokens> {
		constructor(opts: ParserOptions) {
			super(opts, "parse/commit", {});
		}

		tokenize(index: Number0): undefined | TokenValues<Tokens> {
			const char = this.getInputCharOnly(index);
			switch (char) {
				case Symbols.Space:
				case Symbols.Tab: {
					while (
						this.getInputCharOnly(index) === Symbols.Space ||
						this.getInputCharOnly(index) === Symbols.Tab
					) {
						index = ob1Add(index, 1);
					}
					return this.finishToken("Whitespace", index);
				}

				case "(":
					return this.finishToken("LeftParen");

				case ")":
					return this.finishToken("RightParen");

				case "!":
					return this.finishToken("Exclamation");

				case ":":
					return this.finishToken("Colon");

				default:
					return this.finishValueToken("Word", char);
			}
		}

		parse(): CommitRoot {
			const start = this.getPosition();

			let commitType = "";
			if (this.matchToken("Word")) {
				if (
					this.matchToken("Word") &&
					/("|')/.test((this.getToken() as Tokens["Word"]).value)
				) {
					this.eatToken("Word");
				}
				while (
					!this.matchToken("LeftParen") &&
					!this.matchToken("Exclamation") &&
					!this.matchToken("Colon")
				) {
					if (!this.matchToken("Word")) {
						this.unexpectedDiagnostic({
							description: descriptions.COMMIT_PARSER.UNEXPECTED_TOKEN,
						});
						break;
					}
					commitType += (this.getToken() as Tokens["Word"]).value;
					this.nextToken();
				}
				if (
					!this.matchToken("LeftParen") &&
					!this.matchToken("Exclamation") &&
					!this.matchToken("Colon")
				) {
					commitType = "";
				}
			} else {
				this.unexpectedDiagnostic({
					description: descriptions.COMMIT_PARSER.MISSING_TYPE,
				});
			}

			const custom = !/^fix$/i.test(commitType) && !/^feat$/i.test(commitType);

			let scope = "";
			if (this.eatToken("LeftParen")) {
				if (this.matchToken("Word")) {
					while (!this.matchToken("RightParen")) {
						if (this.matchToken("Word")) {
							scope += (this.getToken() as Tokens["Word"]).value;
						} else if (this.matchToken("Whitespace")) {
							scope += " ";
						} else {
							this.unexpectedDiagnostic({
								description: descriptions.COMMIT_PARSER.UNEXPECTED_TOKEN,
							});
							break;
						}
						this.nextToken();
					}
					this.nextToken();
				} else {
					this.unexpectedDiagnostic({
						description: descriptions.COMMIT_PARSER.EMPTY_SCOPE,
					});
				}
			}

			let breaking = !!this.eatToken("Exclamation");

			let rawBody = "";
			if (this.eatToken("Colon")) {
				while (!this.matchToken("EOF")) {
					if (this.eatToken("Whitespace")) {
						rawBody += " ";
						continue;
					}

					if (this.eatToken("LeftParen")) {
						rawBody += "(";
						continue;
					}

					if (this.eatToken("RightParen")) {
						rawBody += ")";
						continue;
					}

					if (this.eatToken("Exclamation")) {
						rawBody += "!";
						continue;
					}

					if (this.eatToken("Colon")) {
						rawBody += ":";
						continue;
					}

					if (this.matchToken("Word")) {
						rawBody += (this.getToken() as Tokens["Word"]).value;
						this.nextToken();
						continue;
					}
				}
			} else {
				this.unexpectedDiagnostic({
					description: descriptions.COMMIT_PARSER.MISSING_DESCRIPTION,
				});
			}

			if (/BREAKING[\-\s]CHANGE:\s\S+/.test(rawBody)) {
				breaking = true;
			}

			return this.finishNode(
				start,
				this.finishRoot({
					type: "CommitRoot",
					breaking,
					commitType: commitType.toLowerCase(),
					custom,
					rawBody: rawBody.trim(),
					scope,
				}),
			);
		}
	}
);
