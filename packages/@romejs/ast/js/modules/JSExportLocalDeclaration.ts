/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ConstExportModuleKind,
	JSClassDeclaration,
	JSExportLocalSpecifier,
	JSFunctionDeclaration,
	JSNodeBase,
	JSVariableDeclarationStatement,
	TSDeclareFunction,
	TSEnumDeclaration,
	TSInterfaceDeclaration,
	TSModuleDeclaration,
	TSTypeAliasTypeAnnotation,
} from "@romejs/ast";
import {createBuilder} from "../utils";

export type JSExportLocalDeclaration = JSNodeBase & {
	type: "JSExportLocalDeclaration";
	declaration?:
		| undefined
		| JSVariableDeclarationStatement
		| JSFunctionDeclaration
		| JSClassDeclaration
		| TSModuleDeclaration
		| TSEnumDeclaration
		| TSTypeAliasTypeAnnotation
		| TSInterfaceDeclaration
		| TSDeclareFunction;
	specifiers?: Array<JSExportLocalSpecifier>;
	exportKind?: ConstExportModuleKind;
	declare?: boolean;
};

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
