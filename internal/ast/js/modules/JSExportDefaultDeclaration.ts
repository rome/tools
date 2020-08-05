/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyJSExpression,
	JSClassDeclaration,
	JSFunctionDeclaration,
	NodeBaseWithComments,
	TSDeclareFunction,
	TSInterfaceDeclaration,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSExportDefaultDeclaration extends NodeBaseWithComments {
	readonly type: "JSExportDefaultDeclaration";
	declaration:
		| JSFunctionDeclaration
		| JSClassDeclaration
		| TSInterfaceDeclaration
		| TSDeclareFunction
		| AnyJSExpression;
	readonly exportKind?: undefined;
	readonly declare?: boolean;
}

export const jsExportDefaultDeclaration = createBuilder<JSExportDefaultDeclaration>(
	"JSExportDefaultDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			declaration: true,
		},
	},
);
