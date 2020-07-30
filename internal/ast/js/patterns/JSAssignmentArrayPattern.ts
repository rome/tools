/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */ import {
	AnyJSAssignmentPattern,
	AnyJSTargetAssignmentPattern,
	JSArrayHole,
	JSPatternMeta,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSAssignmentArrayPattern extends NodeBaseWithComments {
	readonly type: "JSAssignmentArrayPattern";
	readonly meta?: JSPatternMeta;
	readonly elements: Array<JSArrayHole | AnyJSAssignmentPattern>;
	readonly rest?: AnyJSTargetAssignmentPattern;
}

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
