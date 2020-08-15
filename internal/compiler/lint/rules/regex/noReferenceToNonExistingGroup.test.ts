/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"Dangling backslash in regex",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					String.raw`let foo = /([abc]+)=\18/;foo;`,
					String.raw`let foo = /([abc]+)=\49/;foo;`,
					String.raw`let foo = /([abc]+)=\78/;foo;`,
					String.raw`let foo = /([abc]+)=\99/;foo;`,
					String.raw`let foo = /(([abc])\19)+=\28/;foo;`,
					String.raw`let foo = /([abc]+)=\199/;foo;`,
				],
				valid: [
					String.raw`let foo = /([abc]+)=\1/;foo;`,
					// matches first capture group
					String.raw`let foo = /([abc]+)=\2/;foo;`,
					// matches \2 escaped
					String.raw`let foo = /([abc]+)=\8/;foo;`,
					// matches '8'
					String.raw`let foo = /([abc]+)=\9/;foo;`,
					// matches '9'
					String.raw`let foo = /([abc]+)=\119/;foo;`,
					// matches '\t' followd by '9'
					String.raw`let foo = /([abc]+)=\338/;foo;`,
					// matches \33 (char code 255) followed by '8'
					String.raw`let foo = /([abc]+)=\377/;foo;`,
					// matches \377 (char code 255)
					String.raw`let foo = /([abc]+)=\777/;foo;`,
					// matches \77  (char code 63) followed by '7'
				],
				category: "lint/regex/noReferenceToNonExistingGroup",
				filename: "file.ts",
			},
		);
	},
);
