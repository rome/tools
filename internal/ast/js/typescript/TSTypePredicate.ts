/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSPrimary,
	JSIdentifier,
	NodeBaseWithComments,
	TSThisType,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTypePredicate extends NodeBaseWithComments {
	readonly type: "TSTypePredicate";
	readonly asserts: boolean;
	readonly parameterName: JSIdentifier | TSThisType;
	readonly typeAnnotation?: AnyTSPrimary;
}

export const tsTypePredicate = createBuilder<TSTypePredicate>(
	"TSTypePredicate",
	{
		bindingKeys: {},
		visitorKeys: {
			parameterName: true,
			typeAnnotation: true,
		},
	},
);
