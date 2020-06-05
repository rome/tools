/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */ import {
	AnyJSAssignmentPattern,
	AnyJSTargetAssignmentPattern,
	JSArrayHole,
	JSNodeBase,
	JSPatternMeta,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSAssignmentArrayPattern = JSNodeBase & {
	type: "JSAssignmentArrayPattern";
	meta?: JSPatternMeta;
	elements: Array<JSArrayHole | AnyJSAssignmentPattern>;
	rest?: AnyJSTargetAssignmentPattern;
};

export const jsAssignmentArrayPattern = createBuilder<JSAssignmentArrayPattern>(
	"JSAssignmentArrayPattern",
	{
		bindingKeys: {},
		visitorKeys: {
			elements: true,
			rest: true,
			meta: true,
		},
	},
);
