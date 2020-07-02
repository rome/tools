/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSClassMember,
	AnyJSExpression,
	NodeBaseWithComments,
	TSExpressionWithTypeArguments,
	TSTypeParameterDeclaration,
	TSTypeParameterInstantiation,
} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSClassHead = NodeBaseWithComments & {
	type: "JSClassHead";
	superClass?: AnyJSExpression;
	body: Array<AnyJSClassMember>;
	typeParameters?: TSTypeParameterDeclaration;
	superTypeParameters?: TSTypeParameterInstantiation;
	implements?: undefined | Array<TSExpressionWithTypeArguments>;
};

export const jsClassHead = createQuickBuilder<JSClassHead, "body">(
	"JSClassHead",
	"body",
	{
		bindingKeys: {},
		visitorKeys: {
			superClass: true,
			body: true,
			typeParameters: true,
			superTypeParameters: true,
			implements: true,
		},
	},
);
