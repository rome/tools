/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyTSPrimary,
	NodeBaseWithComments,
	TSTypeParameter,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export type TSMappedTypeBoolean = undefined | boolean | "+" | "-";

export interface TSMappedType extends NodeBaseWithComments {
	readonly type: "TSMappedType";
	readonly typeParameter: TSTypeParameter;
	readonly typeAnnotation?: AnyTSPrimary;
	readonly optional?: TSMappedTypeBoolean;
	readonly readonly?: TSMappedTypeBoolean;
}

export const tsMappedType = createBuilder<TSMappedType>(
	"TSMappedType",
	{
		bindingKeys: {},
		visitorKeys: {
			typeParameter: true,
			typeAnnotation: true,
		},
	},
);
