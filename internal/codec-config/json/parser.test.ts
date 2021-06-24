/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/core";
import {descriptions} from "@internal/diagnostics";
import {json} from "@internal/codec-config";
import {test} from "rome";
import {readMarkup} from "@internal/markup";
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
