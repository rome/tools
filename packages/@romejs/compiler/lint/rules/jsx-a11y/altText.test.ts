/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx a11y alt text",
	async (t) => {
		await testLintMultiple(
			t,
			// rome-ignore lint/js/noTemplateCurlyInString
			[
				// INVALID,
				"<img src='foo' />",
				"<img {...props} />",
				"<img {...props} alt={undefined} />",
				"<img src='foo' role='presentation' />",
				"<img src='foo' role='none' />",
				"<object {...props} />",
				"<object aria-label={undefined} />",
				"<area {...props} />",
				"<area alt={undefined} />",
				"<input type='image' {...props} />",
				"<input type='image' {...props} alt={undefined} />",

				// VALID
				"<img {...props} alt />",
				"<img src='foo' alt='Foo eating a sandwich.' />",
				"<img src='foo' alt={'Foo eating a sandwich.'} />",
				"<img src='foo' alt={altText} />",
				"<img src='foo' alt={`${person} smiling`} />",
				"<img src='foo' alt='' />",
				"<object aria-label />",
				"<object aria-label='foo' />",
				"<object aria-labelledby='id1' />",
				"<object>Meaningful description</object>",
				"<object>{hello}</object>",
				"<object title='An object' />",
				"<area {...props} alt />",
				"<area aria-label='foo' />",
				"<area aria-labelledby='id1' />",
				"<area alt='This is descriptive!' />",
				"<input type='image' {...props} alt />",
				"<input type='image' alt='This is descriptive!' />",
				"<input type='image' aria-label='foo' />",
				"<input type='image' aria-labelledby='id1' />",
			],
			{category: "lint/jsx-a11y/altText"},
		);
	},
);
