import {AnyNode} from "@internal/ast";
import {
	Diagnostic,
	DiagnosticCategory,
	DiagnosticSuppressions,
	descriptions,
	formatCategoryDescription,
	splitPossibleCategoryName,
} from "@internal/diagnostics";
import {ZeroIndexed} from "@internal/numbers";
import {
	BaseTokens,
	ParserCore,
	ParserCoreTokenizeState,
	ParserOptions,
	SimpleToken,
	StringToken,
	ValueToken,
	createParser,
	isntLineBreak,
	isntWhitespace,
} from "@internal/parser-core";
import {unescapeJSONString} from "@internal/string-escape";
import {isEscaped} from "@internal/string-utils";

export const SUPPRESSION_START = "rome-ignore";
export const INCORRECT_SUPPRESSION_START = [
	"rome-disable",
	"@rome-ignore",
	"@rome-disable",
	"romefrontend-ignore",
	"romefrontend-disable",
	"@rometools-ignore",
	"@rometools-disable",
];

export type ExtractedSuppressions = {
	suppressions: DiagnosticSuppressions;
	diagnostics: Diagnostic[];
};

type Tokens = BaseTokens & {
	BadPrefixTypo: StringToken<"BadPrefixTypo">;
	BadPrefixMissingSpace: SimpleToken<"BadPrefixMissingSpace">;
	ValidPrefix: SimpleToken<"ValidPrefix">;
	InvalidCategory: ValueToken<"InvalidCategory", string>;
	Category: ValueToken<"Category", DiagnosticCategory>;
	CategoryValue: StringToken<"CategoryValue">;
	Explanation: StringToken<"Explanation">;
};

interface Options extends ParserOptions {
	targetNode: undefined | AnyNode;
	requireExplanations: boolean;
}

interface State {
	searching: boolean;
}

type ParserTypes = {
	tokens: Tokens;
	state: State;
	options: ParserOptions;
	meta: {
		searching: boolean;
	};
};

type SuppressionCommentParser = ParserCore<ParserTypes>;

function isStringValueChar(
	char: string,
	index: ZeroIndexed,
	input: string,
): boolean {
	return !(char === '"' && !isEscaped(index, input));
}

function isCategoryValueChar(
	char: string,
	index: ZeroIndexed,
	input: string,
): boolean {
	return !(char === ")" && !isEscaped(index, input));
}

function isCategoryNameChar(char: string): boolean {
	return char !== "(" && isntWhitespace(char) && char !== ":";
}

const suppressionCommentParser = createParser<ParserTypes>({
	diagnosticCategoryValue: "suppressions",
	diagnosticLanguage: "text",
	ignoreWhitespaceTokens: true,

	getInitialState(parser) {
		return {
			searching: parser.meta.searching,
		};
	},

	tokenizeWithState(
		parser: SuppressionCommentParser,
		index: ZeroIndexed,
		state: State,
	): ParserCoreTokenizeState<ParserTypes> {
		if (state.searching) {
			// Ignore leading stars
			if (parser.getInputCharOnly(index) === "*") {
				return parser.lookahead(index.increment());
			}

			// Get the first word
			const [firstWord, end] = parser.readInputFrom(index, isntWhitespace);

			// Check for prefix typos
			for (const possiblePrefixTypo of INCORRECT_SUPPRESSION_START) {
				if (firstWord === possiblePrefixTypo) {
					return [
						state,
						parser.finishValueToken("BadPrefixTypo", firstWord, end),
					];
				}
			}

			// Not a suppression comment. Skip to the end of the line.
			if (!firstWord.startsWith(SUPPRESSION_START)) {
				const [, end] = parser.readInputFrom(index, isntLineBreak);
				return parser.lookahead(end);
			}

			// Missing space after suppression prefix
			if (firstWord !== SUPPRESSION_START) {
				return [
					state,
					parser.finishToken(
						"BadPrefixMissingSpace",
						index.add(SUPPRESSION_START.length),
					),
				];
			}

			return [
				{
					searching: false,
				},
				parser.finishToken("ValidPrefix", end),
			];
		} else {
			const char = parser.getInputCharOnly(index);

			// If the current character is a colon then we're an explanation
			if (char === ":") {
				const [rawExplanation, end] = parser.readInputFrom(
					index.increment(),
					isntLineBreak,
				);
				const explanation = rawExplanation.trim();

				// Handle the developer being cheeky and having a colon but an empty explanation, it's the same thing mate!
				if (explanation === "") {
					return parser.lookahead(end);
				}

				return [
					{
						searching: false,
					},
					parser.finishValueToken("Explanation", explanation, end),
				];
			}

			// Category value
			if (char === "(") {
				let value = "";
				let valueEnd;

				const [nextChar, innerValueStringStart] = parser.getInputChar(
					index.increment(),
				);
				if (nextChar === '"') {
					// String value we need to escape
					const [rawValue, innerValueEnd] = parser.readInputFrom(
						innerValueStringStart,
						isStringValueChar,
					);

					// Ensure next character is a closing quote
					const [endChar, stringValueEnd] = parser.getInputChar(innerValueEnd);
					if (endChar !== '"') {
						// TODO Unclosed quote
					}

					value = unescapeJSONString(
						rawValue,
						(metadata, strIndex) => {
							throw parser.unexpected({
								description: metadata,
								start: parser.getPositionFromIndex(index.add(strIndex)),
							});
						},
					);
					valueEnd = stringValueEnd;
				} else {
					// Otherwise we can just safely read this until the closing )
					[value, valueEnd] = parser.readInputFrom(
						index.increment(),
						isCategoryValueChar,
					);
				}

				// Ensure next character is a closing )
				const [closeChar, end] = parser.getInputChar(valueEnd);
				if (closeChar !== ")") {
					// TODO Unclosed category value
				}

				return [state, parser.finishValueToken("CategoryValue", value, end)];
			}

			// Read a category name!
			const [strCategoryName, end] = parser.readInputFrom(
				index,
				isCategoryNameChar,
			);

			const categoryName = splitPossibleCategoryName(strCategoryName);

			let token;
			if (categoryName === undefined) {
				token = parser.finishValueToken("InvalidCategory", strCategoryName, end);
			} else {
				token = parser.finishValueToken("Category", categoryName, end);
			}

			return [state, token];
		}
	},
});

export function parseCategoryPair(
	parser: SuppressionCommentParser,
): [
	Tokens["Category"] | Tokens["InvalidCategory"],
	undefined | Tokens["CategoryValue"]
] {
	return [
		parser.eatToken("InvalidCategory") || parser.expectToken("Category"),
		parser.eatToken("CategoryValue"),
	];
}

// Avoid parsing comments without suppressions with a fast regex
const COMMENT_PARSE_CHECK = new RegExp(
	[...INCORRECT_SUPPRESSION_START, SUPPRESSION_START].join("|"),
	"g",
);

const EMPTY_EXTRACTIONS: ExtractedSuppressions = {
	suppressions: [],
	diagnostics: [],
};

export function parseCommentSuppressions(opts: Options): ExtractedSuppressions {
	if (opts.input === undefined || opts.input.search(COMMENT_PARSE_CHECK) === -1) {
		return EMPTY_EXTRACTIONS;
	}

	const parser: SuppressionCommentParser = suppressionCommentParser.create(
		opts,
		{
			searching: true,
		},
	);
	const {requireExplanations, targetNode} = opts;

	const suppressedCategories: Set<string> = new Set();
	const suppressions: DiagnosticSuppressions = [];

	while (!parser.matchToken("EOF")) {
		const token = parser.getToken();

		switch (token.type) {
			case "BadPrefixTypo": {
				parser.unexpectedDiagnostic({
					token,
					description: descriptions.SUPPRESSIONS.INCORRECT_SUPPRESSION_START,
				});
				parser.nextToken();
				break;
			}

			case "BadPrefixMissingSpace": {
				parser.unexpectedDiagnostic({
					token,
					description: descriptions.SUPPRESSIONS.MISSING_SPACE,
				});
				parser.nextToken();
				break;
			}

			case "CategoryValue": {
				// TODO Error: Category value not attached to a category!
				parser.nextToken();
				break;
			}

			case "ValidPrefix": {
				if (targetNode === undefined || targetNode.loc === undefined) {
					parser.unexpectedDiagnostic({
						token,
						description: descriptions.SUPPRESSIONS.MISSING_TARGET,
					});
					parser.nextToken();
					break;
				}

				parser.nextToken();
				const startLine = targetNode.loc.start.line;
				const endLine = targetNode.loc.end.line;

				const categories = [];
				while (
					parser.matchToken("Category") ||
					parser.matchToken("InvalidCategory")
				) {
					categories.push(parseCategoryPair(parser));
				}

				if (categories.length === 0) {
					parser.unexpectedDiagnostic({
						description: descriptions.SUPPRESSIONS.EMPTY,
					});
					parser.nextToken();
					break;
				}

				for (const [categoryToken, categoryValueToken] of categories) {
					let categoryValue = categoryValueToken?.value;
					if (categoryValue === "") {
						categoryValue = undefined;
					}

					const loc = parser.finishLocAt(
						parser.getPositionFromIndex(categoryToken.start),
						parser.getPositionFromIndex(
							(categoryValueToken ?? categoryToken).end,
						),
					);

					if (categoryToken.type === "InvalidCategory") {
						parser.unexpectedDiagnostic({
							description: descriptions.SUPPRESSIONS.INVALID_CATEGORY_NAME(
								categoryToken.value,
							),
							loc,
						});
					} else {
						const category = categoryToken.value;
						const dupeKey = formatCategoryDescription({category, categoryValue});

						if (suppressedCategories.has(dupeKey)) {
							parser.unexpectedDiagnostic({
								token: categoryToken,
								description: descriptions.SUPPRESSIONS.DUPLICATE(dupeKey),
							});
						} else {
							suppressedCategories.add(dupeKey);

							suppressions.push({
								path: parser.path,
								category,
								categoryValue,
								loc,
								startLine,
								endLine,
							});
						}
					}
				}

				if (requireExplanations && !parser.eatToken("Explanation")) {
					parser.unexpectedDiagnostic({
						description: descriptions.SUPPRESSIONS.MISSING_EXPLANATION,
					});
				}

				break;
			}

			default: {
				parser.nextToken();
				break;
			}
		}
	}

	parser.finalize();

	return {
		diagnostics: parser.getDiagnostics(),
		suppressions,
	};
}

export function parseCommentSuppressionLoneCategory(
	opts: ParserOptions,
): {
	category: DiagnosticCategory;
	categoryValue: undefined | string;
} {
	const parser = suppressionCommentParser.create(
		opts,
		{
			searching: false,
		},
	);
	const [category, categoryValue] = parseCategoryPair(parser);
	parser.finalize();

	if (category.type === "InvalidCategory") {
		throw parser.unexpected({
			description: descriptions.SUPPRESSIONS.INVALID_CATEGORY_NAME(
				category.value,
			),
			token: category,
		});
	}

	return {
		category: category.value,
		categoryValue: categoryValue?.value,
	};
}
