/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type JSDirective = NodeBaseWithComments & {
	type: "JSDirective";
	value: string;
};

export const jsDirective = createBuilder<JSDirective>(
	"JSDirective",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
