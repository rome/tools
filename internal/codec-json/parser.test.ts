/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/core";
import {descriptions} from "@internal/diagnostics";
import {parseJSON} from "@internal/codec-json";
import {test} from "rome";
import {ParserOptions} from "@internal/parser-core";
import {createUnknownPath} from "@internal/path";
import {readMarkup} from "@internal/markup";

// These are just some very basic tests, most of it is already covered by test262-parse so most are redundant
function parseExtJSON(opts: ParserOptions) {
	return parseJSON({...opts, path: createUnknownPath("input.rjson")});
}

test(
	"comments",
	(t) => {
		// comment at beginning
		t.true(parseExtJSON({input: "// comment\ntrue"}));
		t.true(parseExtJSON({input: "/* comment */\ntrue"}));
		t.true(parseExtJSON({input: "/* comment */ true"}));

		// comment at end
		t.true(parseExtJSON({input: "true\n// comment"}));
		t.true(parseExtJSON({input: "true\n/* comment */"}));
		t.true(parseExtJSON({input: "true/* comment */"}));

		// comment before object property
		t.looksLike(
			parseExtJSON({input: '{/* comment */ "foo": "bar"}'}),
			{
				foo: "bar",
			},
		);
		t.looksLike(
			parseExtJSON({input: '{// comment\n"foo": "bar"}'}),
			{
				foo: "bar",
			},
		);

		// comment before object property value
		t.looksLike(
			parseExtJSON({input: '{"foo": /* comment */ "bar"}'}),
			{
				foo: "bar",
			},
		);
		t.looksLike(
			parseExtJSON({input: '{"foo": // comment\n"bar"}'}),
			{
				foo: "bar",
			},
		);

		// comment after object property value
		t.looksLike(
			parseExtJSON({input: '{"foo": "bar" /* comment */,}'}),
			{
				foo: "bar",
			},
		);
		t.looksLike(
			parseExtJSON({input: '{"foo": "bar" // comment\n,}'}),
			{
				foo: "bar",
			},
		);

		// comment after object property
		t.looksLike(
			parseExtJSON({input: '{"foo": "bar", /* comment */}'}),
			{
				foo: "bar",
			},
		);
		t.looksLike(
			parseExtJSON({input: '{"foo": "bar", // comment\n}'}),
			{
				foo: "bar",
			},
		);

		// comment before array element
		t.looksLike(parseExtJSON({input: '[/* comment */ "foo"]'}), ["foo"]);
		t.looksLike(parseExtJSON({input: '[//comment\n"foo"]'}), ["foo"]);

		// comment after array element
		t.looksLike(parseExtJSON({input: '["foo" /* comment */]'}), ["foo"]);
		t.looksLike(parseExtJSON({input: '["foo" //comment\n]'}), ["foo"]);

		// comment after array element value
		t.looksLike(
			parseExtJSON({input: '["foo" /* comment */, "bar"]'}),
			["foo", "bar"],
		);
		t.looksLike(
			parseExtJSON({input: '["foo" //comment\n, "bar"]'}),
			["foo", "bar"],
		);

		// comment only in array
		t.looksLike(parseExtJSON({input: "[/* comment */]"}), []);
		t.looksLike(parseExtJSON({input: "[// comment\n]"}), []);

		// comment only in object
		t.looksLike(parseExtJSON({input: "{/* comment */}"}), {});
		t.looksLike(parseExtJSON({input: "{// comment\n}"}), {});

		// ensure closed block comment
		t.throws(
			() => {
				parseExtJSON({input: "true /* unclosed comment"});
			},
			readMarkup(descriptions.JSON.UNCLOSED_BLOCK_COMMENT.message),
		);
	},
);

test(
	"numbers",
	(t) => {
		t.is(parseExtJSON({input: "1"}), 1);
		t.is(parseExtJSON({input: "12"}), 12);
		t.is(parseExtJSON({input: "123"}), 123);
		t.is(parseExtJSON({input: "1.2"}), 1.2);
		t.is(parseExtJSON({input: "1234.21234"}), 1_234.21234);
		t.is(parseExtJSON({input: "0.5e+5"}), 50_000);
		t.is(parseExtJSON({input: "0.5e-5"}), 0.000005);
		t.is(parseExtJSON({input: "0.5E+5"}), 50_000);
		t.is(parseExtJSON({input: "0.5E-5"}), 0.000005);
	},
);

test(
	"strings",
	(t) => {
		t.is(parseExtJSON({input: '"foo"'}), "foo");
		t.is(parseExtJSON({input: '"foo\u1234"'}), "foo\u1234");
		t.is(parseExtJSON({input: '"foo\\n"'}), "foo\n");
		t.is(parseExtJSON({input: '"foo\\t"'}), "foo\t");
		t.is(parseExtJSON({input: '"foo\n"'}), "foo\n");
		t.is(parseExtJSON({input: '"foo\t"'}), "foo\t");

		t.throws(
			() => {
				parseExtJSON({input: '"foo'});
			},
			readMarkup(descriptions.JSON.UNCLOSED_STRING.message),
		);

		t.throws(
			() => {
				parseExtJSON({input: "'foo'"});
			},
			readMarkup(descriptions.JSON.SINGLE_QUOTE_USAGE.message),
		);

		t.throws(
			() => {
				parseExtJSON({input: '"\\u000Z"'});
			},
			readMarkup(
				descriptions.STRING_ESCAPE.INVALID_HEX_DIGIT_FOR_ESCAPE.message,
			),
		);

		t.throws(
			() => {
				parseExtJSON({input: '"\\u123"'});
			},
			readMarkup(descriptions.STRING_ESCAPE.NOT_ENOUGH_CODE_POINTS.message),
		);
	},
);

test(
	"booleans",
	(t) => {
		t.is(parseExtJSON({input: "true"}), true);
		t.is(parseExtJSON({input: "false"}), false);
	},
);

test(
	"null",
	(t) => {
		t.is(parseExtJSON({input: "null"}), null);
	},
);

test(
	"undefined",
	(t) => {
		t.throws(
			() => {
				t.is(parseExtJSON({input: "undefined"}), undefined);
			},
			readMarkup(descriptions.JSON.UNDEFINED_IN_JSON.message),
		);
	},
);

test(
	"arrays",
	(t) => {
		t.looksLike(parseExtJSON({input: "[]"}), []);
		t.looksLike(parseExtJSON({input: "[1, 2, 3]"}), [1, 2, 3]);
		t.looksLike(parseExtJSON({input: "[[1, 2, 3]]"}), [[1, 2, 3]]);

		t.throws(
			() => {
				parseExtJSON({input: "[,]"});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);

		t.throws(
			() => {
				parseExtJSON({input: "[1,,]"});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);

		t.throws(
			() => {
				parseExtJSON({input: "[1, /*comment*/,]"});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);

		t.throws(
			() => {
				parseExtJSON({input: '["foo": "bar"]'});
			},
			readMarkup(descriptions.JSON.MISTAKEN_ARRAY_IDENTITY.message),
		);
	},
);

test(
	"objects",
	(t) => {
		t.looksLike(parseExtJSON({input: "{}"}), {});
		t.looksLike(parseExtJSON({input: '{"foo": "bar"}'}), {foo: "bar"});
		t.looksLike(
			parseExtJSON({input: '{"foo": "bar", "bar": "foo"}'}),
			{
				foo: "bar",
				bar: "foo",
			},
		);

		t.throws(
			() => {
				parseExtJSON({input: "{,}"});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);

		t.throws(
			() => {
				parseExtJSON({input: '{"foo": "bar",,}'});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);

		t.throws(
			() => {
				parseExtJSON({input: '{"foo": "bar", /*comment*/,}'});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);
	},
);

test(
	"regular JSON",
	(t) => {
		t.throws(
			() => {
				parseJSON({input: '{foo: "bar"}'});
			},
			readMarkup(descriptions.JSON.PROPERTY_KEY_UNQUOTED_IN_JSON.message),
		);

		t.throws(
			() => {
				parseJSON({input: "// foobar\ntrue"});
			},
			readMarkup(descriptions.JSON.COMMENTS_IN_JSON.message),
		);

		t.throws(
			() => {
				parseJSON({input: "/* foobar */\ntrue"});
			},
			readMarkup(descriptions.JSON.COMMENTS_IN_JSON.message),
		);

		t.throws(
			() => {
				parseJSON({input: '{"foo": "bar",}'});
			},
			readMarkup(descriptions.JSON.TRAILING_COMMA_IN_JSON.message),
		);

		t.throws(
			() => {
				parseJSON({input: '["foo",]'});
			},
			readMarkup(descriptions.JSON.TRAILING_COMMA_IN_JSON.message),
		);
	},
);
