/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */ import {
	AnyJSParamBindingPattern,
	AnyJSTargetBindingPattern,
	JSArrayHole,
	JSNodeBase,
	JSPatternMeta,
} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSBindingArrayPattern = JSNodeBase & {
	type: "JSBindingArrayPattern";
	meta?: JSPatternMeta;
	elements: Array<JSArrayHole | AnyJSParamBindingPattern>;
	rest: undefined | AnyJSTargetBindingPattern;
};

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
