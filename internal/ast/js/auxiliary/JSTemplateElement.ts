/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSTemplateElement extends NodeBaseWithComments {
	readonly type: "JSTemplateElement";
	readonly tail?: boolean;
	readonly cooked: string;
	readonly raw: string;
}

export const jsTemplateElement = createBuilder<JSTemplateElement>(
	"JSTemplateElement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
