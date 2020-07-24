/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConstTSAccessibility, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface JSClassPropertyMeta extends NodeBaseWithComments {
	type: "JSClassPropertyMeta";
	static?: boolean;
	accessibility?: ConstTSAccessibility;
	optional?: boolean;
	readonly?: boolean;
	abstract?: boolean;
}

export const jsClassPropertyMeta = createBuilder<JSClassPropertyMeta>(
	"JSClassPropertyMeta",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
