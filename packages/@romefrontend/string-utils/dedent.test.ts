/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {dedent} from "./dedent";
import {test} from "rome";

test(
	"dedent",
	(t) => {
		t.is(dedent("\tx\n\ty"), "x\ny");

		t.is(dedent("\tx\n\t\ty\n\tz"), "x\n\ty\nz");

		t.is(
			dedent`
        if (x) {
          y = ${"1"};
        }
      `,
			"if (x) {\n  y = 1;\n}",
		);
	},
);
