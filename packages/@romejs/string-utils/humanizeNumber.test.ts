/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {humanizeNumber} from "./humanizeNumber";
import {test} from "rome";

test(
	"humanizeNumber",
	(t) => {
		t.is(humanizeNumber(0), "0");
		t.is(humanizeNumber(500), "500");
		t.is(humanizeNumber(1_000), "1_000");
		t.is(humanizeNumber(10_000), "10_000");
		t.is(humanizeNumber(100_000), "100_000");
		t.is(humanizeNumber(1_000_000), "1_000_000");
		t.is(humanizeNumber(10_000_000), "10_000_000");

		t.is(humanizeNumber(0, ","), "0");
		t.is(humanizeNumber(500, ","), "500");
		t.is(humanizeNumber(1_000, ","), "1,000");
		t.is(humanizeNumber(10_000, ","), "10,000");
		t.is(humanizeNumber(100_000, ","), "100,000");
		t.is(humanizeNumber(1_000_000, ","), "1,000,000");
		t.is(humanizeNumber(10_000_000, ","), "10,000,000");
	},
);
