/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createQuickBuilder} from "../../utils";

export interface JSAssignmentIdentifier extends NodeBaseWithComments {
	readonly type: "JSAssignmentIdentifier";
	readonly name: string;
	readonly definite?: boolean;
}

export const jsAssignmentIdentifier = createQuickBuilder<
	JSAssignmentIdentifier,
	"name"
>(
	"JSAssignmentIdentifier",
	"name",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
