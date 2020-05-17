/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {dedent} from "@romejs/string-utils";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx key",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"const a = [<div />, <div />]",
				dedent(
					`
          const a = [1, 2].map(x => <div>{x}</div>);
        `,
				),
				dedent(
					`
          const a = [1, 2].map(x => {
            return <div>{x}</div>;
          });
        `,
				),
				dedent(
					`
          const a = [1, 2].map(function(x) {
            return <div>{x}</div>;
          });
        `,
				),
				// VALID
				'const a = [<div key="a" />, <div key={"b"} />]',
				dedent(
					`
          const a = [1, 2].map(x => <div key={x}>{x}</div>);
        `,
				),
				dedent(
					`
          const a = [1, 2].map(x => {
            return <div key={x}>{x}</div>;
          });
        `,
				),
				dedent(
					`
          const a = [1, 2].map(function(x) {
            return <div key={x}>{x}</div>;
          });
        `,
				),
			],
			{category: "lint/jsxKey"},
		);
	},
);
