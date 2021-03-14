/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/core";
import {descriptions} from "@internal/diagnostics";
import {json, rjson} from "@internal/codec-config";
import {test} from "rome";
import {readMarkup} from "@internal/markup";

test(
	"comments",
	(t) => {
		// comment at beginning
		t.true(rjson.parse({input: "// comment\ntrue"}) === true);
		t.true(rjson.parse({input: "/* comment */\ntrue"}) === true);
		t.true(rjson.parse({input: "/* comment */ true"}) === true);

		// comment at end
		t.true(rjson.parse({input: "true\n// comment"}) === true);
		t.true(rjson.parse({input: "true\n/* comment */"}) === true);
		t.true(rjson.parse({input: "true/* comment */"}) === true);

		// comment before object property
		t.looksLike(
			rjson.parse({input: '{/* comment */ "foo": "bar"}'}),
			{
				foo: "bar",
			},
		);
		t.looksLike(
			rjson.parse({input: '{// comment\n"foo": "bar"}'}),
			{
				foo: "bar",
			},
		);

		// comment before object property value
		t.looksLike(
			rjson.parse({input: '{"foo": /* comment */ "bar"}'}),
			{
				foo: "bar",
			},
		);
		t.looksLike(
			rjson.parse({input: '{"foo": // comment\n"bar"}'}),
			{
				foo: "bar",
			},
		);

		// comment after object property value
		t.looksLike(
			rjson.parse({input: '{"foo": "bar" /* comment */,}'}),
			{
				foo: "bar",
			},
		);
		t.looksLike(
			rjson.parse({input: '{"foo": "bar" // comment\n,}'}),
			{
				foo: "bar",
			},
		);

		// comment after object property
		t.looksLike(
			rjson.parse({input: '{"foo": "bar", /* comment */}'}),
			{
				foo: "bar",
			},
		);
		t.looksLike(
			rjson.parse({input: '{"foo": "bar", // comment\n}'}),
			{
				foo: "bar",
			},
		);

		// comment before array element
		t.looksLike(rjson.parse({input: '[/* comment */ "foo"]'}), ["foo"]);
		t.looksLike(rjson.parse({input: '[//comment\n"foo"]'}), ["foo"]);

		// comment after array element
		t.looksLike(rjson.parse({input: '["foo" /* comment */]'}), ["foo"]);
		t.looksLike(rjson.parse({input: '["foo" //comment\n]'}), ["foo"]);

		// comment after array element value
		t.looksLike(
			rjson.parse({input: '["foo" /* comment */, "bar"]'}),
			["foo", "bar"],
		);
		t.looksLike(
			rjson.parse({input: '["foo" //comment\n, "bar"]'}),
			["foo", "bar"],
		);

		// comment only in array
		t.looksLike(rjson.parse({input: "[/* comment */]"}), []);
		t.looksLike(rjson.parse({input: "[// comment\n]"}), []);

		// comment only in object
		t.looksLike(rjson.parse({input: "{/* comment */}"}), {});
		t.looksLike(rjson.parse({input: "{// comment\n}"}), {});

		// ensure closed block comment
		t.throws(
			() => {
				rjson.parse({input: "true /* unclosed comment"});
			},
			readMarkup(descriptions.JSON.UNCLOSED_BLOCK_COMMENT.message),
		);
	},
);

test(
	"numbers",
	(t) => {
		t.is(rjson.parse({input: "1"}), 1);
		t.is(rjson.parse({input: "12"}), 12);
		t.is(rjson.parse({input: "123"}), 123);
		t.is(rjson.parse({input: "1.2"}), 1.2);
		t.is(rjson.parse({input: "1234.21234"}), 1_234.21234);
		t.is(rjson.parse({input: "0.5e+5"}), 50_000);
		t.is(rjson.parse({input: "0.5e-5"}), 0.000005);
		t.is(rjson.parse({input: "0.5E+5"}), 50_000);
		t.is(rjson.parse({input: "0.5E-5"}), 0.000005);
	},
);

test(
	"strings",
	(t) => {
		t.is(rjson.parse({input: '"foo"'}), "foo");
		t.is(rjson.parse({input: '"foo\u1234"'}), "foo\u1234");
		t.is(rjson.parse({input: '"foo\\n"'}), "foo\n");
		t.is(rjson.parse({input: '"foo\\t"'}), "foo\t");
		t.is(rjson.parse({input: '"foo\n"'}), "foo\n");
		t.is(rjson.parse({input: '"foo\t"'}), "foo\t");

		t.throws(
			() => {
				rjson.parse({input: '"foo'});
			},
			readMarkup(descriptions.JSON.UNCLOSED_STRING.message),
		);

		t.throws(
			() => {
				rjson.parse({input: "'foo'"});
			},
			readMarkup(descriptions.JSON.SINGLE_QUOTE_USAGE.message),
		);

		t.throws(
			() => {
				rjson.parse({input: '"\\u000Z"'});
			},
			readMarkup(
				descriptions.STRING_ESCAPE.INVALID_HEX_DIGIT_FOR_ESCAPE.message,
			),
		);

		t.throws(
			() => {
				rjson.parse({input: '"\\u123"'});
			},
			readMarkup(descriptions.STRING_ESCAPE.NOT_ENOUGH_CODE_POINTS.message),
		);
	},
);

test(
	"booleans",
	(t) => {
		t.is(rjson.parse({input: "true"}), true);
		t.is(rjson.parse({input: "false"}), false);
	},
);

test(
	"null",
	(t) => {
		t.is(rjson.parse({input: "null"}), null);
	},
);

test(
	"undefined",
	(t) => {
		t.throws(
			() => {
				t.is(rjson.parse({input: "undefined"}), undefined);
			},
			readMarkup(descriptions.JSON.UNDEFINED_IN_JSON.message),
		);
	},
);

test(
	"arrays",
	(t) => {
		t.looksLike(rjson.parse({input: "[]"}), []);
		t.looksLike(rjson.parse({input: "[1, 2, 3]"}), [1, 2, 3]);
		t.looksLike(rjson.parse({input: "[[1, 2, 3]]"}), [[1, 2, 3]]);

		t.throws(
			() => {
				rjson.parse({input: "[,]"});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);

		t.throws(
			() => {
				rjson.parse({input: "[1,,]"});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);

		t.throws(
			() => {
				rjson.parse({input: "[1, /*comment*/,]"});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);

		t.throws(
			() => {
				rjson.parse({input: '["foo": "bar"]'});
			},
			readMarkup(descriptions.JSON.MISTAKEN_ARRAY_IDENTITY.message),
		);
	},
);

test(
	"objects",
	(t) => {
		t.looksLike(rjson.parse({input: "{}"}), {});
		t.looksLike(rjson.parse({input: '{"foo": "bar"}'}), {foo: "bar"});
		t.looksLike(
			rjson.parse({input: '{"foo": "bar", "bar": "foo"}'}),
			{
				foo: "bar",
				bar: "foo",
			},
		);

		t.throws(
			() => {
				rjson.parse({input: "{,}"});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);

		t.throws(
			() => {
				rjson.parse({input: '{"foo": "bar",,}'});
			},
			readMarkup(descriptions.JSON.REDUNDANT_COMMA.message),
		);

		t.throws(
			() => {
				rjson.parse({input: '{"foo": "bar", /*comment*/,}'});
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
				json.parse({input: '{foo: "bar"}'});
			},
			readMarkup(descriptions.JSON.PROPERTY_KEY_UNQUOTED_IN_JSON.message),
		);

		t.throws(
			() => {
				json.parse({input: "// foobar\ntrue"});
			},
			readMarkup(descriptions.JSON.COMMENTS_IN_JSON.message),
		);

		t.throws(
			() => {
				json.parse({input: "/* foobar */\ntrue"});
			},
			readMarkup(descriptions.JSON.COMMENTS_IN_JSON.message),
		);

		t.throws(
			() => {
				json.parse({input: '{"foo": "bar",}'});
			},
			readMarkup(descriptions.JSON.TRAILING_COMMA_IN_JSON.message),
		);

		t.throws(
			() => {
				json.parse({input: '["foo",]'});
			},
			readMarkup(descriptions.JSON.TRAILING_COMMA_IN_JSON.message),
		);
	},
);
