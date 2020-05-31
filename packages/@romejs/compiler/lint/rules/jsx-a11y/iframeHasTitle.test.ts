/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";

import {testLintMultiple} from "../testHelpers";

test(
	"jsx-a11y iframe has title",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<iframe />",
				"<iframe {...props} />",
				'<iframe title="" />',
				'<iframe title={""} />',
				"<iframe title={``} />",
				"<iframe title={undefined} />",
				"<iframe title={false} />",
				"<iframe title={true} />",
				"<iframe title={42} />",
				// VALID
				'<iframe title="title" />',
				"<iframe title={title} >",
			],
			{category: "lint/jsx-a11y/iframeHasTitle"},
		);
	},
);
