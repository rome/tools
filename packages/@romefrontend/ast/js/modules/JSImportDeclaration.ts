/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ConstJSImportModuleKind,
	JSImportDefaultSpecifier,
	JSImportNamespaceSpecifier,
	JSImportSpecifier,
	JSStringLiteral,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type AnyImportSpecifier =
	| JSImportDefaultSpecifier
	| JSImportNamespaceSpecifier
	| JSImportSpecifier;

export type JSImportDeclaration = NodeBaseWithComments & {
	type: "JSImportDeclaration";
	defaultSpecifier?: JSImportDefaultSpecifier;
	namespaceSpecifier?: JSImportNamespaceSpecifier;
	namedSpecifiers: Array<JSImportSpecifier>;
	source: JSStringLiteral;
	importKind?: ConstJSImportModuleKind;
};

export const jsImportDeclaration = createBuilder<JSImportDeclaration>(
	"JSImportDeclaration",
	{
		bindingKeys: {
			defaultSpecifier: true,
			namespaceSpecifier: true,
			namedSpecifiers: true,
		},
		visitorKeys: {
			defaultSpecifier: true,
			namespaceSpecifier: true,
			namedSpecifiers: true,
			source: true,
		},
	},
);
