import {test} from "rome";
import {
	markupToken,
	reduceParserCore,
} from "@internal/markup-syntax-highlight/utils";
import {ob1Coerce0} from "@internal/ob1";
import {StaticMarkup} from "@internal/markup";
import {ReduceCallbackResult} from "./types";

test(
	"should craft markup for tokens",
	async (t) => {
		const tokenInput = makeStaticMarkup([
			fakeToken,
			makeRawMarkup(fakeToken),
			makeStaticMarkup([fakeToken, makeRawMarkup(fakeToken)]),
		]);

		const expectedMarkupOutput = makeStaticMarkup([
			expectedMarkupPrefix,
			makeStaticMarkup([
				makeStaticMarkup([
					fakeToken,
					makeRawMarkup(fakeToken),
					makeStaticMarkup([fakeToken, makeRawMarkup(fakeToken)]),
				]),
			]),
			expectedMarkupSufix,
		]);

		t.looksLike(markupToken(validTokenType, tokenInput), expectedMarkupOutput);
	},
);

const validTokenType = "keyword";
const fakeToken = "lorem";

function makeRawMarkup(word: string) {
	return (<MarkupPart>{
		type: "RAW_MARKUP",
		value: word,
	});
}

// hacky way of extracting a private type
const dummyPart = (<StaticMarkup>{
	type: "MARKUP",
	parts: [{type: "RAW_MARKUP", value: "lorem"}],
}).parts[0];
type MarkupPart = typeof dummyPart;

function makeStaticMarkup(parts: MarkupPart[]) {
	return (<StaticMarkup>{
		type: "MARKUP",
		parts,
	});
}

const expectedMarkupPrefix = (<MarkupPart>{
	type: "RAW_MARKUP",
	value: `<token type=\"${validTokenType}\">`,
});
const expectedMarkupSufix = (<MarkupPart>{
	type: "RAW_MARKUP",
	value: "</token>",
});

test(
	"should identify and markup tokens from input string",
	async (t) => {
		const fakeTokensCount = 10;
		const fakeTokens = Array.from(Array(fakeTokensCount).keys()).map((i) => ({
			type: `token${i}`,
			start: ob1Coerce0(i * 7),
			end: ob1Coerce0(i * 7 + 6),
		}));
		const fakeInput = `${fakeTokens.map((token) => token.type).join(" ")} invalid`;
		const expectedOutput = [
			makeStaticMarkup([
				makeRawMarkup(
					`${fakeTokens.map((token) =>
						`<token type=\"${validTokenType}\">${token.type}</token>`
					).join(" ")} <emphasis><color bg="red">invalid</color></emphasis>`,
				),
			]),
		];

		const result = reduceParserCore(
			fakeInput,
			[
				...fakeTokens,
				{
					type: "Invalid",
					start: ob1Coerce0(fakeTokensCount * 7),
					end: ob1Coerce0(fakeTokensCount * 7 + 7),
				},
				{
					type: "EOF",
					start: ob1Coerce0(fakeTokensCount * 7 + 8),
					end: ob1Coerce0(fakeTokensCount * 7 + 9),
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

				return (<ReduceCallbackResult>{
					type: validTokenType,
					value: makeStaticMarkup([value]),
				});
			},
		);
		t.looksLike(result, expectedOutput);
	},
);
