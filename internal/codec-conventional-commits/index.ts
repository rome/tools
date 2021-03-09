import {descriptions} from "@internal/diagnostics";
import {ParserOptions, TokenValues, createParser} from "@internal/parser-core";
import {ConventionalCommit, Symbols, Tokens} from "./types";

type CommitParserTypes = {
	tokens: Tokens;
	state: {};
	options: ParserOptions;
	meta: void;
};

export {ConventionalCommit as Commit};

const commitParser = createParser<CommitParserTypes>({
	diagnosticLanguage: "text",
	diagnosticCategoryValue: "commit",

	tokenize(parser, index) {
		const char = parser.getInputCharOnly(index);
		switch (char) {
			case Symbols.Space:
			case Symbols.Tab: {
				while (
					parser.getInputCharOnly(index) === Symbols.Space ||
					parser.getInputCharOnly(index) === Symbols.Tab
				) {
					index = index.increment();
				}
				return parser.finishToken("Whitespace", index);
			}

			case "(":
				return parser.finishToken("LeftParen");

			case ")":
				return parser.finishToken("RightParen");

			case "!":
				return parser.finishToken("Exclamation");

			case ":":
				return parser.finishToken("Colon");

			default:
				return parser.finishValueToken("Word", char);
		}
	},
});

export function parseConventionalCommit(opts: ParserOptions): ConventionalCommit {
	const parser = commitParser.create(opts);

	let commitType = "";
	if (parser.matchToken("Word")) {
		if (
			parser.matchToken("Word") &&
			/(["'])/.test((parser.getToken() as Tokens["Word"]).value)
		) {
			parser.eatToken("Word");
		}
		while (
			!(parser.matchToken("LeftParen") ||
			parser.matchToken("Exclamation") ||
			parser.matchToken("Colon"))
		) {
			if (!parser.matchToken("Word")) {
				parser.unexpectedDiagnostic({
					description: descriptions.COMMIT_PARSER.UNEXPECTED_TOKEN,
				});
				break;
			}
			commitType += (parser.getToken() as Tokens["Word"]).value;
			parser.nextToken();
		}
		if (
			!(parser.matchToken("LeftParen") ||
			parser.matchToken("Exclamation") ||
			parser.matchToken("Colon"))
		) {
			commitType = "";
		}
	} else {
		parser.unexpectedDiagnostic({
			description: descriptions.COMMIT_PARSER.MISSING_TYPE,
		});
	}

	const custom = !(/^fix$/i.test(commitType) || /^feat$/i.test(commitType));

	let scope = "";
	if (parser.eatToken("LeftParen")) {
		if (parser.matchToken("Word")) {
			while (!parser.matchToken("RightParen")) {
				if (parser.matchToken("Word")) {
					scope += parser.expectToken("Word").value;
				} else if (parser.matchToken("Whitespace")) {
					scope += " ";
				} else {
					parser.unexpectedDiagnostic({
						description: descriptions.COMMIT_PARSER.UNEXPECTED_TOKEN,
					});
					break;
				}
				parser.nextToken();
			}
			parser.nextToken();
		} else {
			parser.unexpectedDiagnostic({
				description: descriptions.COMMIT_PARSER.EMPTY_SCOPE,
			});
		}
	}

	let breaking = parser.eatToken("Exclamation") !== undefined;

	let rawBody = "";
	if (parser.eatToken("Colon")) {
		rawBody = parser.input.slice(parser.getIndex().valueOf());
	} else {
		parser.unexpectedDiagnostic({
			description: descriptions.COMMIT_PARSER.MISSING_DESCRIPTION,
		});
	}

	if (/BREAKING[\-\s]CHANGE:\s\S+/.test(rawBody)) {
		breaking = true;
	}

	return {
		diagnostics: parser.getDiagnostics(),
		breaking,
		commitType: commitType.toLowerCase(),
		custom,
		rawBody: rawBody.trim(),
		scope,
	};
}

export function tokenizeCommit(opts: ParserOptions): TokenValues<Tokens>[] {
	return commitParser.create(opts).getAllTokens();
}
