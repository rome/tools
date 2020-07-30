/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */ import {
	AnyJSParamBindingPattern,
	AnyJSTargetBindingPattern,
	JSArrayHole,
	JSPatternMeta,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSBindingArrayPattern extends NodeBaseWithComments {
	readonly type: "JSBindingArrayPattern";
	readonly meta?: JSPatternMeta;
	readonly elements: Array<JSArrayHole | AnyJSParamBindingPattern>;
	readonly rest: undefined | AnyJSTargetBindingPattern;
}

export const jsBindingArrayPattern = createBuilder<JSBindingArrayPattern>(
	"JSBindingArrayPattern",
	{
		bindingKeys: {
			elements: true,
			rest: true,
		},
		visitorKeys: {
			elements: true,
			rest: true,
			meta: true,
		},
	},
);
