/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSIdentifier, JSNodeBase, TSThisType} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSTypePredicate = JSNodeBase & {
	type: "TSTypePredicate";
	asserts: boolean;
	parameterName: JSIdentifier | TSThisType;
	typeAnnotation?: AnyTSPrimary;
};

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
