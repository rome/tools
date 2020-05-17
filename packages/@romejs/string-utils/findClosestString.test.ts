/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {findClosestStringMatch} from "./findClosestStringMatch";
import {test} from "rome";

test(
	"findClosestStringMatch",
	(t) => {
		t.is(
			findClosestStringMatch("french", ["quebec", "123", "france", "frenc"]),
			"frenc",
		);
		t.is(
			findClosestStringMatch("iphone", ["ipod", "iphone 5s", "iphones x"]),
			"iphone 5s",
		);

		t.is(
			findClosestStringMatch(
				"french",
				["quebec", "123", "france", "frenc"],
				0.9,
			),
			undefined,
		);
		t.is(
			findClosestStringMatch("iphone", ["ipod", "iphone 5s", "iphones x"], 0.9),
			undefined,
		);
	},
);
