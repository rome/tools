/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSNodeBase, TSTypeParameter} from "../index";
import {createBuilder} from "../utils";

export type TSMappedTypeBoolean = undefined | boolean | "+" | "-";

export type TSMappedType = JSNodeBase & {
	type: "TSMappedType";
	typeParameter: TSTypeParameter;
	typeAnnotation?: AnyTSPrimary;
	optional?: TSMappedTypeBoolean;
	readonly?: TSMappedTypeBoolean;
};

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
