/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/cli-layout";
import {sortSemverVersions, stringifySemver} from "@internal/codec-semver";
import {test} from "rome";

test(
	"sort",
	(t) => {
		const sorted = sortSemverVersions(["5.3.6", "1.2.3", "3.2.1", "1.2.4"]);

		t.is(stringifySemver(sorted[0]), "1.2.3");
		t.is(stringifySemver(sorted[1]), "1.2.4");
		t.is(stringifySemver(sorted[2]), "3.2.1");
		t.is(stringifySemver(sorted[3]), "5.3.6");
	},
);
