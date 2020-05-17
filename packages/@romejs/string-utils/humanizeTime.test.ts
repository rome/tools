/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {humanizeTime} from "./humanizeTime";
import {test} from "rome";

test(
	"humanizeTime",
	(t) => {
		t.is(humanizeTime(1), "0.00s");
		t.is(humanizeTime(10), "0.01s");
		t.is(humanizeTime(100), "0.10s");
		t.is(humanizeTime(1_000), "1s");
		t.is(humanizeTime(10_000), "10s");
		t.is(humanizeTime(100_000), "1m40s");
		t.is(humanizeTime(1_000_000), "16m40s");
		t.is(humanizeTime(10_000_000), "2h46m40s");
		t.is(humanizeTime(100_000_000), "27h46m40s");

		t.is(humanizeTime(1, true), "1ms");
		t.is(humanizeTime(10, true), "10ms");
		t.is(humanizeTime(100, true), "100ms");
		t.is(humanizeTime(1_000, true), "1s");
		t.is(humanizeTime(10_000, true), "10s");
		t.is(humanizeTime(100_000, true), "1m40s");
		t.is(humanizeTime(1_000_000, true), "16m40s");
		t.is(humanizeTime(10_000_000, true), "2h46m40s");
		t.is(humanizeTime(100_000_000, true), "27h46m40s");
	},
);
