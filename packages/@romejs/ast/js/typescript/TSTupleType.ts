/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSNodeBase, TSOptionalType} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSTupleType = JSNodeBase & {
	type: "TSTupleType";
	elementTypes: Array<AnyTSPrimary | TSOptionalType>;
	rest?: AnyTSPrimary;
};

export const tsTupleType = createBuilder<TSTupleType>(
	"TSTupleType",
	{
		bindingKeys: {},
		visitorKeys: {
			elementTypes: true,
			rest: true,
		},
	},
);
