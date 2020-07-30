/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MockParent extends NodeBaseWithComments {
	readonly type: "MockParent";
}

export const jsMockParent = createBuilder<MockParent>(
	"MockParent",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);

export const MOCK_PARENT: MockParent = {
	type: "MockParent",
};
