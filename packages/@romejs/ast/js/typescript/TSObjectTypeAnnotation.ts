/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSTypeElement, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSObjectTypeAnnotation = NodeBaseWithComments & {
	type: "TSObjectTypeAnnotation";
	members: Array<AnyTSTypeElement>;
};

export const tsObjectTypeAnnotation = createBuilder<TSObjectTypeAnnotation>(
	"TSObjectTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {
			members: true,
		},
	},
);
