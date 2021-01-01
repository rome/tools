/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTemplateElement extends NodeBaseWithComments {
	readonly type: "TSTemplateElement";
	readonly tail?: boolean;
	readonly cooked: string;
	readonly raw: string;
}

export const tsTemplateElement = createBuilder<TSTemplateElement>(
	"TSTemplateElement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
