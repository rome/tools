import {test} from "rome";
import {
	markupToken,
	reduceParserCore,
} from "@internal/markup-syntax-highlight/utils";
import {StaticMarkup} from "@internal/markup";
import {convertToMarkupFromRandomString} from "@internal/markup/escape";
import {ZeroIndexed} from "@internal/numbers";

const validTokenType = "keyword";
const fakeToken = "lorem";

test(
	"should craft markup for tokens",
	async (t) => {
		const tokenInput: StaticMarkup = [
			fakeToken,
			convertToMarkupFromRandomString(fakeToken),
			[fakeToken, convertToMarkupFromRandomString(fakeToken)],
		];

		t.inlineSnapshot(
			markupToken(validTokenType, tokenInput),
			'[\n\tRAW_MARKUP {value: "<token type=\\"keyword\\">"}\n\t["lorem", RAW_MARKUP {value: "lorem"}, ["lorem", RAW_MARKUP {value: "lorem"}]]\n\tRAW_MARKUP {value: "</token>"}\n]',
		);
	},
);

test(
	"should identify and markup tokens from input string",
	async (t) => {
		const fakeTokensCount = 10;
		const fakeTokens = Array.from(Array(fakeTokensCount).keys()).map((i) => ({
			type: `token${i}`,
			start: new ZeroIndexed(i * 7),
			end: new ZeroIndexed(i * 7 + 6),
		}));
		const fakeInput = `${fakeTokens.map((token) => token.type).join(" ")} invalid`;

		const result = reduceParserCore(
			fakeInput,
			[
				...fakeTokens,
				{
					type: "Invalid",
					start: new ZeroIndexed(fakeTokensCount * 7),
					end: new ZeroIndexed(fakeTokensCount * 7 + 7),
				},
				{
					type: "EOF",
					start: new ZeroIndexed(fakeTokensCount * 7 + 8),
					end: new ZeroIndexed(fakeTokensCount * 7 + 9),
				},
			],
			(token, value, prev, next) => {
				const tokenId = parseInt(token.type[token.type.length - 1]);
				const prevTokenId =
					prev === undefined
						? undefined
						: parseInt(prev.type[prev.type.length - 1]);
				const nextTokenId =
					next === undefined
						? undefined
						: parseInt(next.type[next.type.length - 1]);

				if (tokenId === 0) {
					t.is(prevTokenId, undefined);
					t.is(nextTokenId, 1);
				} else if (tokenId === fakeTokensCount - 1) {
					t.is(prevTokenId, fakeTokensCount - 2);
					t.is(next!.type, "Invalid");
				} else {
					t.is(prevTokenId, tokenId - 1);
					t.is(nextTokenId, tokenId + 1);
				}

				return {
					type: validTokenType,
					value: [value],
				};
			},
		);

		t.inlineSnapshot(
			result,
			'[\n\tRAW_MARKUP {\n\t\tvalue: "<token type=\\"keyword\\">token0</token> <token type=\\"keyword\\">token1</token> <token type=\\"keyword\\">token2</token> <token type=\\"keyword\\">token3</token> <token type=\\"keyword\\">token4</token> <token type=\\"keyword\\">token5</token> <token type=\\"keyword\\">token6</token> <token type=\\"keyword\\">token7</token> <token type=\\"keyword\\">token8</token> <token type=\\"keyword\\">token9</token> <emphasis><color bg=\\"red\\">invalid</color></emphasis>"\n\t}\n]',
		);
	},
);
