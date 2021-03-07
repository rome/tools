/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/markup";
import {test} from "rome";
import {markup} from "./escape";
import {parseMarkup} from "./parse";

test(
	"should not parse string escapes",
	async (t) => {
		t.snapshot(markup`<filelink target="C:\\Users\\sebmck\\file.ts" />`);

		t.snapshot(
			parseMarkup(
				markup`<info>[MemoryFileSystem] Adding new project directory C:\\Users\\sebmck\\rome</info>`,
			),
		);

		t.snapshot(
			parseMarkup(
				markup`  \\<info>[MemoryFileSystem] Adding new project directory C:\\\\Users\\\\Sebastian\\\\rome\\\\\\</info>\n        <error><emphasis>^</emphasis></error> `,
			),
		);
	},
);
