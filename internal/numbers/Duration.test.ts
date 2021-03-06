/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Duration from "./Duration";
import {test} from "rome";

test(
	"Duration.format",
	(t) => {
		t.inlineSnapshot(Duration.fromMilliseconds(1).format(), "1ms");
		t.inlineSnapshot(Duration.fromMilliseconds(10).format(), "10ms");
		t.inlineSnapshot(Duration.fromMilliseconds(100).format(), "0.10s");
		t.inlineSnapshot(Duration.fromMilliseconds(1_000).format(), "1s");
		t.inlineSnapshot(Duration.fromMilliseconds(10_000).format(), "10s");
		t.inlineSnapshot(Duration.fromMilliseconds(100_000).format(), "1m40s");
		t.inlineSnapshot(Duration.fromMilliseconds(1_000_000).format(), "16m40s");
		t.inlineSnapshot(Duration.fromMilliseconds(10_000_000).format(), "2h46m40s");
		t.inlineSnapshot(
			Duration.fromMilliseconds(100_000_000).format(),
			"1d3h46m40s",
		);

		const longOpts = {longform: true};
		t.inlineSnapshot(
			Duration.fromMilliseconds(1).format(longOpts),
			"1 millisecond",
		);
		t.inlineSnapshot(
			Duration.fromMilliseconds(10).format(longOpts),
			"10 milliseconds",
		);
		t.inlineSnapshot(
			Duration.fromMilliseconds(100).format(longOpts),
			"0.10 seconds",
		);
		t.inlineSnapshot(
			Duration.fromMilliseconds(1_000).format(longOpts),
			"1 second",
		);
		t.inlineSnapshot(
			Duration.fromMilliseconds(10_000).format(longOpts),
			"10 seconds",
		);
		t.inlineSnapshot(
			Duration.fromMilliseconds(100_000).format(longOpts),
			"1 minute 40 seconds",
		);
		t.inlineSnapshot(
			Duration.fromMilliseconds(1_000_000).format(longOpts),
			"16 minutes 40 seconds",
		);
		t.inlineSnapshot(
			Duration.fromMilliseconds(10_000_000).format(longOpts),
			"2 hours 46 minutes 40 seconds",
		);
		t.inlineSnapshot(
			Duration.fromMilliseconds(100_000_000).format(longOpts),
			"1 day 3 hours 46 minutes 40 seconds",
		);
	},
);
