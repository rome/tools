/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSNodeBase} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSUnionTypeAnnotation = JSNodeBase & {
	type: "TSUnionTypeAnnotation";
	types: Array<AnyTSPrimary>;
};

export const tsUnionTypeAnnotation = createBuilder<TSUnionTypeAnnotation>(
	"TSUnionTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {
			types: true,
		},
	},
);
