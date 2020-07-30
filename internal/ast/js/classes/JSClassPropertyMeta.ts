/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConstTSAccessibility, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSClassPropertyMeta extends NodeBaseWithComments {
	readonly type: "JSClassPropertyMeta";
	readonly static?: boolean;
	readonly accessibility?: ConstTSAccessibility;
	readonly optional?: boolean;
	readonly readonly?: boolean;
	readonly abstract?: boolean;
}

export const jsClassPropertyMeta = createBuilder<JSClassPropertyMeta>(
	"JSClassPropertyMeta",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
