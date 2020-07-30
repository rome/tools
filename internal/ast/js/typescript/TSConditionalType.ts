/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSConditionalType extends NodeBaseWithComments {
	readonly type: "TSConditionalType";
	readonly checkType: AnyTSPrimary;
	readonly extendsType: AnyTSPrimary;
	readonly trueType: AnyTSPrimary;
	readonly falseType: AnyTSPrimary;
}

export const tsConditionalType = createBuilder<TSConditionalType>(
	"TSConditionalType",
	{
		bindingKeys: {},
		visitorKeys: {
			checkType: true,
			extendsType: true,
			trueType: true,
			falseType: true,
		},
	},
);
