/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../testHelpers";
import {dedent} from "@romejs/string-utils";

test(
	"getter return",
	async (t) => {
		await testLint(
			t,
			dedent`
        class p {
          get name() {
            console.log('hello');
          }
        }
      `,
			{category: "lint/getterReturn"},
		);

		await testLint(
			t,
			dedent`
        let p = {
          get name() {
            console.log('hello');
          },
        };
      `,
			{category: "lint/getterReturn"},
		);

		await testLint(
			t,
			dedent`
        let p = {};
        Object.defineProperty(p, {
          get: function () {
            console.log('hello');
          },
        });
      `,
			{category: "lint/getterReturn"},
		);
	},
);
