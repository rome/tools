/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ConstJSExportModuleKind,
	JSClassDeclaration,
	JSExportLocalSpecifier,
	JSFunctionDeclaration,
	JSVariableDeclarationStatement,
	NodeBaseWithComments,
	TSDeclareFunction,
	TSEnumDeclaration,
	TSInterfaceDeclaration,
	TSModuleDeclaration,
	TSTypeAlias,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface JSExportLocalDeclaration extends NodeBaseWithComments {
	readonly type: "JSExportLocalDeclaration";
	declaration?:
		| undefined
		| JSVariableDeclarationStatement
		| JSFunctionDeclaration
		| JSClassDeclaration
		| TSModuleDeclaration
		| TSEnumDeclaration
		| TSTypeAlias
		| TSInterfaceDeclaration
		| TSDeclareFunction;
	readonly specifiers?: Array<JSExportLocalSpecifier>;
	readonly exportKind?: ConstJSExportModuleKind;
	readonly declare?: boolean;
}

export const jsExportLocalDeclaration = createBuilder<JSExportLocalDeclaration>(
	"JSExportLocalDeclaration",
	{
		bindingKeys: {
			declaration: true,
		},
		visitorKeys: {
			declaration: true,
			specifiers: true,
		},
	},
);
