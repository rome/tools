/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSParamBindingPattern,
	AnyJSTargetBindingPattern,
	AnyTSPrimary,
	JSBindingIdentifier,
	NodeBaseWithComments,
	TSTypeParameterDeclaration,
} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSFunctionHead extends NodeBaseWithComments {
	readonly type: "JSFunctionHead";
	readonly params: Array<AnyJSParamBindingPattern>;
	readonly rest?: AnyJSTargetBindingPattern;
	readonly thisType?: JSBindingIdentifier;
	readonly hasHoistedVars?: boolean;
	readonly generator?: boolean;
	readonly async?: boolean;
	readonly typeParameters?: TSTypeParameterDeclaration;
	readonly returnType?: AnyTSPrimary;
}

export const jsFunctionHead = createQuickBuilder<JSFunctionHead, "params">(
	"JSFunctionHead",
	"params",
	{
		bindingKeys: {
			params: true,
			rest: true,
		},
		visitorKeys: {
			params: true,
			thisType: true,
			rest: true,
			returnType: true,
			typeParameters: true,
		},
	},
);
