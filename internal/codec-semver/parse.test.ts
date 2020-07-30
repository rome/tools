/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/cli-layout";
import {parseSemverRange, parseSemverVersion} from "@internal/codec-semver";
import {test} from "rome";

test(
	"parse",
	async (t) => {
		// Versions in version mode
		const versionTests = [
			"1.2.3",
			"1.2.3-prerelease",
			"1.2.3-pre.2",
			"1.2.3-pre.2.3.4.5.foo",
			"1.2.3+build",
			"1.2.3-prerelease+build",
			"1.2.3-pre.2+build",
			"1.2.3-pre.2.3.4.5.foo+build",
			"1.2.3-pre.2.3.4.5.foo+build.2.3.4.foo",
			"1.2.3-45pre.42yes+45build",
		];
		for (const str of versionTests) {
			t.snapshot(parseSemverVersion({input: str}));
		}

		// Loose mode ranges only
		const looseRangeTests = [
			"* - 1.2.3",
			"v1.2.3",
			"||",
			"",
			"1.2.3prerelease",
			"^4.0.0 || ^4.0.0-beta.* || ^3.8.1 || ^3.0.0 || ^3.0.0-rc.0 || ^2.1.0 || ^1.13.1",
		];
		for (const str of looseRangeTests) {
			t.snapshot(parseSemverRange({input: str, loose: true}));

			t.throws(() => {
				parseSemverRange({input: str, loose: false});
			});
		}

		// ranges in range mode
		const rangeTests = [
			// partial versions
			"1",
			"1.2",
			// wildcards
			"1.*",
			"1.*.3",
			"1.2.*",
			"1.x",
			"1.x.3",
			"1.2.x",
			"1.X",
			"1.X.3",
			"1.2.X",
			// ranges
			"1.2.3 - 1.2.4",
			// or
			"1.2 || 3",
			"1 || 2 || 3",
		];

		// operators in range mode
		const operatorTests = [
			">=1.4.5",
			"<=1.4.5",
			">1.4.5",
			"<1.4.5",
			"^1.4.5",
			"~1.4.5",
			">=1.4",
			"<=1.4",
			">1.4",
			"<1.4",
			"^1.4",
			"~1.4",
			">=1",
			"<=1",
			">1",
			"<1",
			"^1",
			"~1",
		];
		for (const op of operatorTests) {
			rangeTests.push(op);
			rangeTests.push(`${op} ${op}`);
		}

		// Run range tests
		for (const str of rangeTests) {
			t.snapshot(parseSemverRange({input: str}));
		}

		// Ensure ranges throw in version mode
		for (const str of rangeTests) {
			t.throws(() => {
				parseSemverVersion({input: str});
			});
		}
	},
);
