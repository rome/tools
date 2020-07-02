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
	TSInterfaceDeclaration,
} from "@romejs/ast";
import {createBuilder} from "../../utils";
import {TSDeclareFunction} from "../typescript/TSDeclareFunction";

export type JSExportDefaultDeclaration = NodeBaseWithComments & {
	type: "JSExportDefaultDeclaration";
	declaration:
		| JSFunctionDeclaration
		| JSClassDeclaration
		| TSInterfaceDeclaration
		| TSDeclareFunction
		| AnyJSExpression;
	exportKind?: undefined;
	declare?: boolean;
};

export const jsExportDefaultDeclaration = createBuilder<JSExportDefaultDeclaration>(
	"JSExportDefaultDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			declaration: true,
		},
	},
);
