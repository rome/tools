/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSAssignmentIdentifier,
	JSAssignmentObjectPatternProperty,
	JSPatternMeta,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSAssignmentObjectPattern extends NodeBaseWithComments {
	readonly type: "JSAssignmentObjectPattern";
	readonly meta?: JSPatternMeta;
	readonly properties: Array<JSAssignmentObjectPatternProperty>;
	readonly rest: undefined | JSAssignmentIdentifier;
}

export const jsAssignmentObjectPattern = createBuilder<JSAssignmentObjectPattern>(
	"JSAssignmentObjectPattern",
	{
		bindingKeys: {},
		visitorKeys: {
			properties: true,
			rest: true,
			meta: true,
		},
	},
);
