import "@romefrontend/core";
import {test} from "rome";
import {tokenizeCSS} from ".";

test(
	"whitespace",
	(t) => {
		const results = [
			{
				type: "Whitespace",
				end: 2,
				start: 0,
			},
			{
				type: "AtKeyword",
				value: "import",
				end: 9,
				start: 2,
			},
			{
				type: "EOF",
				end: 9,
				start: 9,
			},
		];
		t.looksLike(tokenizeCSS({input: "  @import"}), results);
		t.looksLike(tokenizeCSS({input: "\t\t@import"}), results);
		t.looksLike(tokenizeCSS({input: "\n\n@import"}), results);
	},
);

test(
	"quote",
	(t) => {
		function getResults(index: number) {
			return [
				{
					type: "String",
					value: "foo",
					end: index,
					start: 0,
				},
				{
					type: "EOF",
					end: index,
					start: index,
				},
			];
		}
		t.looksLike(tokenizeCSS({input: '"foo"'}), getResults(5));
		t.looksLike(tokenizeCSS({input: '"fo\\6f"'}), getResults(7));
		t.looksLike(
			tokenizeCSS({input: '"fo\n"'}),
			[
				{
					type: "BadString",
					end: 3,
					start: 0,
				},
				{
					type: "Whitespace",
					end: 4,
					start: 3,
				},
				{
					type: "String",
					value: "",
					end: 5,
					start: 4,
				},
				{
					type: "EOF",
					end: 5,
					start: 5,
				},
			],
		);
	},
);

test(
	"apostrophe",
	(t) => {
		function getResults(index: number) {
			return [
				{
					type: "String",
					value: "foo",
					end: index,
					start: 0,
				},
				{
					type: "EOF",
					end: index,
					start: index,
				},
			];
		}
		t.looksLike(tokenizeCSS({input: "'foo'"}), getResults(5));
		t.looksLike(tokenizeCSS({input: "'fo\\6f'"}), getResults(7));
		t.looksLike(
			tokenizeCSS({input: "'fo\n'"}),
			[
				{
					type: "BadString",
					end: 3,
					start: 0,
				},
				{
					type: "Whitespace",
					end: 4,
					start: 3,
				},
				{
					type: "String",
					value: "",
					end: 5,
					start: 4,
				},
				{
					type: "EOF",
					end: 5,
					start: 5,
				},
			],
		);
	},
);

test(
	"hash",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "#foo"}),
			[
				{
					type: "Hash",
					value: "foo",
					hashType: "id",
					end: 4,
					start: 0,
				},
				{
					type: "EOF",
					end: 4,
					start: 4,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "# "}),
			[
				{
					type: "Delim",
					value: "#",
					end: 1,
					start: 0,
				},
				{
					type: "Whitespace",
					end: 2,
					start: 1,
				},
				{
					type: "EOF",
					end: 2,
					start: 2,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "#\\66oo"}),
			[
				{
					type: "Hash",
					value: "foo",
					hashType: "id",
					end: 6,
					start: 0,
				},
				{
					type: "EOF",
					end: 6,
					start: 6,
				},
			],
		);
	},
);

test(
	"left-paren",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "("}),
			[
				{
					type: "LeftParen",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	"right-paren",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "("}),
			[
				{
					type: "LeftParen",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	"+",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "+123"}),
			[
				{
					type: "Number",
					value: 123,
					end: 4,
					start: 0,
					numberType: "integer",
				},
				{
					type: "EOF",
					end: 4,
					start: 4,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "+0.123"}),
			[
				{
					type: "Number",
					value: 0.123,
					end: 6,
					start: 0,
					numberType: "number",
				},
				{
					type: "EOF",
					end: 6,
					start: 6,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "+.123"}),
			[
				{
					type: "Number",
					value: 0.123,
					end: 5,
					start: 0,
					numberType: "number",
				},
				{
					type: "EOF",
					end: 5,
					start: 5,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "+10e3"}),
			[
				{
					type: "Number",
					value: 10_000,
					end: 5,
					start: 0,
					numberType: "integer",
				},
				{
					type: "EOF",
					end: 5,
					start: 5,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "+3.4e-2"}),
			[
				{
					type: "Number",
					value: 0.034,
					end: 7,
					start: 0,
					numberType: "number",
				},
				{
					type: "EOF",
					end: 7,
					start: 7,
				},
			],
		);
	},
);

test(
	"comma",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: ","}),
			[
				{
					type: "Comma",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	"-",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "-foo"}),
			[
				{
					type: "Ident",
					value: "-foo",
					end: 4,
					start: 0,
				},
				{
					type: "EOF",
					end: 4,
					start: 4,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "-123"}),
			[
				{
					type: "Number",
					value: -123,
					end: 4,
					start: 0,
					numberType: "integer",
				},
				{
					type: "EOF",
					end: 4,
					start: 4,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "-0.123"}),
			[
				{
					type: "Number",
					value: -0.123,
					end: 6,
					start: 0,
					numberType: "number",
				},
				{
					type: "EOF",
					end: 6,
					start: 6,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "-.123"}),
			[
				{
					type: "Number",
					value: -0.123,
					end: 5,
					start: 0,
					numberType: "number",
				},
				{
					type: "EOF",
					end: 5,
					start: 5,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "-10e3"}),
			[
				{
					type: "Number",
					value: -10_000,
					end: 5,
					start: 0,
					numberType: "integer",
				},
				{
					type: "EOF",
					end: 5,
					start: 5,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "-3.4e-2"}),
			[
				{
					type: "Number",
					value: -0.034,
					end: 7,
					start: 0,
					numberType: "number",
				},
				{
					type: "EOF",
					end: 7,
					start: 7,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "-->"}),
			[
				{
					type: "CDC",
					end: 3,
					start: 0,
				},
				{
					type: "EOF",
					end: 3,
					start: 3,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "-"}),
			[
				{
					type: "Delim",
					value: "-",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	".",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: ".123"}),
			[
				{
					type: "Number",
					value: 0.123,
					end: 4,
					start: 0,
					numberType: "number",
				},
				{
					type: "EOF",
					end: 4,
					start: 4,
				},
			],
		);
	},
);

test(
	"colon",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: ":"}),
			[
				{
					type: "Colon",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	"semi",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: ";"}),
			[
				{
					type: "Semi",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	"at-import",
	(t) => {
		function getResults(index: number, value: string) {
			return [
				{
					type: "AtKeyword",
					value,
					end: index,
					start: 0,
				},
				{
					type: "EOF",
					end: index,
					start: index,
				},
			];
		}
		t.looksLike(tokenizeCSS({input: "@import"}), getResults(7, "import"));
		t.looksLike(tokenizeCSS({input: "@\\69mport"}), getResults(9, "import"));
		t.looksLike(tokenizeCSS({input: "@\\0"}), getResults(3, "\ufffd"));
	},
);

test(
	"left-square-bracket",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "["}),
			[
				{
					type: "LeftSquareBracket",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	"\\",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "\\66oo"}),
			[
				{
					type: "Ident",
					value: "foo",
					end: 5,
					start: 0,
				},
				{
					type: "EOF",
					end: 5,
					start: 5,
				},
			],
		);
	},
);

test(
	"digit",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "3"}),
			[
				{
					type: "Number",
					value: 3,
					numberType: "integer",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	"right-square-bracket",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "]"}),
			[
				{
					type: "RightSquareBracket",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	"left-curly-bracket",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "{"}),
			[
				{
					type: "LeftCurlyBracket",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	"right-curly-bracket",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "}"}),
			[
				{
					type: "RightCurlyBracket",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);

test(
	"ident",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "foo"}),
			[
				{
					type: "Ident",
					value: "foo",
					end: 3,
					start: 0,
				},
				{
					type: "EOF",
					end: 3,
					start: 3,
				},
			],
		);
	},
);

test(
	"url",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "url(foo)"}),
			[
				{
					type: "URL",
					value: "foo",
					end: 8,
					start: 0,
				},
				{
					type: "EOF",
					end: 8,
					start: 8,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "url('foo')"}),
			[
				{
					type: "Function",
					value: "url",
					end: 4,
					start: 0,
				},
				{
					type: "String",
					value: "foo",
					end: 9,
					start: 4,
				},
				{
					type: "RightParen",
					end: 10,
					start: 9,
				},
				{
					type: "EOF",
					end: 10,
					start: 10,
				},
			],
		);
		t.looksLike(
			tokenizeCSS({input: "url(   foo)"}),
			[
				{
					type: "URL",
					value: "foo",
					end: 11,
					start: 0,
				},
				{
					type: "EOF",
					end: 11,
					start: 11,
				},
			],
		);
	},
);

test(
	"arbitrary delim",
	(t) => {
		t.looksLike(
			tokenizeCSS({input: "^"}),
			[
				{
					type: "Delim",
					value: "^",
					end: 1,
					start: 0,
				},
				{
					type: "EOF",
					end: 1,
					start: 1,
				},
			],
		);
	},
);
