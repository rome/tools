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
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSClassProperty extends NodeBaseWithComments {
	readonly type: "JSClassProperty";
	readonly key: AnyJSObjectPropertyKey;
	readonly meta: JSClassPropertyMeta;
	readonly value?: AnyJSExpression;
	readonly typeAnnotation?: AnyTSPrimary;
	readonly definite?: boolean;
}

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
