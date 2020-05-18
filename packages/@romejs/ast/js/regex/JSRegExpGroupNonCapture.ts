/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyJSRegExpExpression, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../utils";

export type JSRegExpGroupNonCapture = JSNodeBase & {
	type: "JSRegExpGroupNonCapture";
	expression: AnyJSRegExpExpression;
	kind?:
		| "positive-lookahead"
		| "negative-lookahead"
		| "positive-lookbehind"
		| "negative-lookbehind";
};

export const jsRegExpGroupNonCapture = createBuilder<JSRegExpGroupNonCapture>(
	"JSRegExpGroupNonCapture",
	{
		bindingKeys: {},
		visitorKeys: {
			expression: true,
		},
	},
);
