/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from "@romejs/ast";
import {createQuickBuilder} from "../utils";

export type JSAssignmentIdentifier = JSNodeBase & {
	type: "JSAssignmentIdentifier";
	name: string;
	definite?: boolean;
};

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
