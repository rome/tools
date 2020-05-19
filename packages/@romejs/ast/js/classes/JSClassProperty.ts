/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyJSObjectPropertyKey,
	AnyTSPrimary,
	JSClassPropertyMeta,
	JSNodeBase,
} from "@romejs/ast";
import {createBuilder} from "../utils";

export type JSClassProperty = JSNodeBase & {
	type: "JSClassProperty";
	key: AnyJSObjectPropertyKey;
	meta: JSClassPropertyMeta;
	value?: AnyJSExpression;
	typeAnnotation?: AnyTSPrimary;
	definite?: boolean;
};

export const jsClassProperty = createBuilder<JSClassProperty>(
	"JSClassProperty",
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			meta: true,
			value: true,
			typeAnnotation: true,
		},
	},
);
