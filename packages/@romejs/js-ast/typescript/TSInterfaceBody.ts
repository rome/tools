/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSTypeElement, JSNodeBase} from "../index";
import {createBuilder} from "../utils";

export type TSInterfaceBody = JSNodeBase & {
	type: "TSInterfaceBody";
	body: Array<AnyTSTypeElement>;
};

export const tsInterfaceBody = createBuilder<TSInterfaceBody>(
	"TSInterfaceBody",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
