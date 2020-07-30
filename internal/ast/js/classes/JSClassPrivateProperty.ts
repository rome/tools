/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	AnyTSPrimary,
	JSClassPropertyMeta,
	JSPrivateName,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSClassPrivateProperty extends NodeBaseWithComments {
	readonly type: "JSClassPrivateProperty";
	readonly key: JSPrivateName;
	readonly meta: JSClassPropertyMeta;
	readonly value: undefined | AnyJSExpression;
	readonly typeAnnotation?: AnyTSPrimary;
}

export const jsClassPrivateProperty = createBuilder<JSClassPrivateProperty>(
	"JSClassPrivateProperty",
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
