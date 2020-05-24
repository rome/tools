/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"react void dom elements no children",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<br className={'dont-remove-class'}>invalid children</br>",
				"<img>invalid children</img>",
				"<hr children={'invalid children'} />",
				"<area dangerouslySetInnerHTML={{__html: ''}}></area>",
				"<img dangerouslySetInnerHTML={{__html: ''}} children={'invalid children'}>invalid children</img>",
				'React.createElement("img", {children: "child"})',
				'React.createElement("img", {dangerouslySetInnerHTML: {__html: "child"}})',
				'React.createElement("img", {}, "child")',
				'createElement("img", {children: "child"})',
				'createElement("img", {dangerouslySetInnerHTML: {__html: "child"}})',
				'createElement("img", {}, "child")',
				// VALID
				"<div>Children</div>",
				"<div children='Children' />",
				"<div dangerouslySetInnerHTML={{ __html: '' }} />",
				"<br className='valid' />",
				'React.createElement("img", {})',
				'React.createElement("div", {dangerouslySetInnerHTML: {__html: "child"}})',
				'React.createElement("div", {}, "child")',
				'createElement("img", {})',
				'createElement("div", {dangerouslySetInnerHTML: {__html: "child"}})',
				'createElement("div", {}, "child")',
			],
			{category: "lint/react/voidDomElementsNoChildren"},
		);
	},
);
