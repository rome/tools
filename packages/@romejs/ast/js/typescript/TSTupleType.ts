/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments, TSTupleElement} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSTupleType = NodeBaseWithComments & {
	type: "TSTupleType";
	elementTypes: Array<TSTupleElement>;
	rest?: TSTupleElement;
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
