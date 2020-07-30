/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/cli-layout";
import {parseSemverRange, stringifySemver} from "@internal/codec-semver";
import {test} from "rome";

test(
	"stringify",
	(t) => {
		// basic version and whitespace
		t.is(stringifySemver(parseSemverRange({input: "1.2.3"})), "1.2.3");
		t.is(stringifySemver(parseSemverRange({input: "    1.2.3"})), "1.2.3");
		t.is(stringifySemver(parseSemverRange({input: "1.2.3    "})), "1.2.3");
		t.is(stringifySemver(parseSemverRange({input: "\t\t1.2.3\r\n"})), "1.2.3");

		// retains prerelease and build
		t.is(
			stringifySemver(parseSemverRange({input: "1.2.3-prerelease"})),
			"1.2.3-prerelease",
		);
		t.is(
			stringifySemver(parseSemverRange({input: "1.2.3+build"})),
			"1.2.3+build",
		);
		t.is(
			stringifySemver(parseSemverRange({input: "1.2.3-prerelease+build"})),
			"1.2.3-prerelease+build",
		);

		// comparators
		t.is(stringifySemver(parseSemverRange({input: "~1.2.3"})), "~1.2.3");
		t.is(stringifySemver(parseSemverRange({input: "~ 1.2.3"})), "~1.2.3");

		// wildcards
		t.is(stringifySemver(parseSemverRange({input: "1.2"})), "1.2");
		t.is(stringifySemver(parseSemverRange({input: "1.*.3"})), "1.*.3");
		t.is(stringifySemver(parseSemverRange({input: "1.x"})), "1");

		// ranges
		t.is(
			stringifySemver(parseSemverRange({input: "1.2.3 - 1.3.4"})),
			"1.2.3 - 1.3.4",
		);

		// logical and
		t.is(
			stringifySemver(parseSemverRange({input: "1.2.3 >= 1.3.4"})),
			"1.2.3 >=1.3.4",
		);

		// logical or
		t.is(
			stringifySemver(parseSemverRange({input: "1.2.3 || 1.3.4"})),
			"1.2.3 || 1.3.4",
		);
	},
);
