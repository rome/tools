/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase, JSXIdentifier} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type JSXNamespacedName = JSNodeBase & {
	type: "JSXNamespacedName";
	namespace: JSXIdentifier;
	name: JSXIdentifier;
};

export const jsxNamespacedName = createBuilder<JSXNamespacedName>(
	"JSXNamespacedName",
	{
		bindingKeys: {},
		visitorKeys: {
			namespace: true,
			name: true,
		},
	},
);
