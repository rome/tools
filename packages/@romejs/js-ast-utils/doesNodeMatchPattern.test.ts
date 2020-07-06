/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {doesNodeMatchPattern} from "./doesNodeMatchPattern";
import {template} from "./template";

test(
	"doesNodeMatchPattern",
	(t) => {
		t.inlineSnapshot(
			doesNodeMatchPattern(template.expression`foo`, "foo"),
			true,
		);

		t.inlineSnapshot(
			doesNodeMatchPattern(template.expression`this.foo`, "this.foo"),
			true,
		);

		t.inlineSnapshot(
			doesNodeMatchPattern(template.expression`exports.foo`, "exports.**"),
			true,
		);

		t.inlineSnapshot(
			doesNodeMatchPattern(template.expression`this.foo.bar`, "this.foo.*"),
			true,
		);

		t.inlineSnapshot(
			doesNodeMatchPattern(template.expression`this.foo.bar.yes`, "this.foo.*"),
			false,
		);

		t.inlineSnapshot(
			doesNodeMatchPattern(template.expression`this.foo.bar.yes`, "this.foo.**"),
			true,
		);
	},
);
