/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"prefer template",
	async (t) => {
		await testLint(
			t,
			{
				valid: [`console.log('foo' + 'bar')`],
				invalid: [
					`const foo = 'bar'; console.log(foo + 'baz')`,
					`console.log((1 * 2) + 'foo')`,
					`console.log(1 + 'foo' + 2 + 'bar' + 'baz' + 3)`,
					`console.log((1 + 'foo') * 2)`,
					`console.log((1 * (2 + 'foo')) + 'bar')`,
					`console.log(\`foo\` + 1)`,
					`console.log('foo' + \`bar\${\`baz\${'bat' + 'bam'}\`}\` + 'boo')`,
					`console.log('foo' + 1 + 2)`,
				],
			},
			{
				category: "lint/js/preferTemplate",
			},
		);
	},
);
