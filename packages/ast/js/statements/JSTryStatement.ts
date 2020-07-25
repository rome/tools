/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSBlockStatement,
	JSCatchClause,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface JSTryStatement extends NodeBaseWithComments {
	type: "JSTryStatement";
	block: JSBlockStatement;
	handler: undefined | JSCatchClause;
	finalizer: undefined | JSBlockStatement;
}

export const jsTryStatement = createBuilder<JSTryStatement>(
	"JSTryStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			block: true,
			handler: true,
			finalizer: true,
		},
	},
);
