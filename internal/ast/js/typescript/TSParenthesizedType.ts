/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSParenthesizedType extends NodeBaseWithComments {
	readonly type: "TSParenthesizedType";
	readonly typeAnnotation: AnyTSPrimary;
}

export const tsParenthesizedType = createBuilder<TSParenthesizedType>(
	"TSParenthesizedType",
	{
		bindingKeys: {},
		visitorKeys: {
			typeAnnotation: true,
		},
	},
);
