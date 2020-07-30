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
} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSClassHead extends NodeBaseWithComments {
	readonly type: "JSClassHead";
	readonly superClass?: AnyJSExpression;
	readonly body: Array<AnyJSClassMember>;
	readonly typeParameters?: TSTypeParameterDeclaration;
	readonly superTypeParameters?: TSTypeParameterInstantiation;
	readonly implements?: undefined | Array<TSExpressionWithTypeArguments>;
}

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
