/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type TSIntersectionTypeAnnotation = NodeBaseWithComments & {
	type: "TSIntersectionTypeAnnotation";
	types: Array<AnyTSPrimary>;
};

export const tsIntersectionTypeAnnotation = createBuilder<TSIntersectionTypeAnnotation>(
	"TSIntersectionTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {
			types: true,
		},
	},
);
