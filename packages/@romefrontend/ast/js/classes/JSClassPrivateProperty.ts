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
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type JSClassPrivateProperty = NodeBaseWithComments & {
	type: "JSClassPrivateProperty";
	key: JSPrivateName;
	meta: JSClassPropertyMeta;
	value: undefined | AnyJSExpression;
	typeAnnotation?: AnyTSPrimary;
};

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
