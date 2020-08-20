/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {parseDependencyPattern} from "./dependencies";
import {consumeUnknown} from "@internal/consume";

test(
	"can parse npm dependency patterns",
	async (t) => {
		t.snapshot(
			parseDependencyPattern(consumeUnknown("npm:foo", "parse/json"), false),
		);
		t.snapshot(
			parseDependencyPattern(
				consumeUnknown("npm:@foo/bar", "parse/json"),
				false,
			),
		);
		t.snapshot(
			parseDependencyPattern(
				consumeUnknown("npm:foo@1.0.0", "parse/json"),
				false,
			),
		);
		t.snapshot(
			parseDependencyPattern(
				consumeUnknown("npm:@foo/bar@1.0.0", "parse/json"),
				false,
			),
		);
	},
);

test(
	"can parse gist patterns",
	async (t) => {
		t.snapshot(
			parseDependencyPattern(consumeUnknown("gist:123456", "parse/json"), false),
		);
	},
);

test(
	"can parse workspace patterns",
	async (t) => {
		t.snapshot(
			parseDependencyPattern(consumeUnknown("workspace:*", "parse/json"), false),
		);
	},
);
