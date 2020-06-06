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
	JSNodeBase,
	TSTypeParameterDeclaration,
} from "@romejs/ast";
import {createQuickBuilder} from "../../utils";

export type JSFunctionHead = JSNodeBase & {
	type: "JSFunctionHead";
	params: Array<AnyJSParamBindingPattern>;
	rest?: AnyJSTargetBindingPattern;
	thisType?: JSBindingIdentifier;
	hasHoistedVars?: boolean;
	generator?: boolean;
	async?: boolean;
	typeParameters?: TSTypeParameterDeclaration;
	returnType?: AnyTSPrimary;
};

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
