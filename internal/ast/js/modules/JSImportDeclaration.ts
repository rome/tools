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
} from "@internal/ast";
import {createBuilder} from "../../utils";

export type AnyImportSpecifier =
	| JSImportDefaultSpecifier
	| JSImportNamespaceSpecifier
	| JSImportSpecifier;

export interface JSImportDeclaration extends NodeBaseWithComments {
	readonly type: "JSImportDeclaration";
	readonly defaultSpecifier?: JSImportDefaultSpecifier;
	readonly namespaceSpecifier?: JSImportNamespaceSpecifier;
	readonly namedSpecifiers: Array<JSImportSpecifier>;
	readonly source: JSStringLiteral;
	readonly importKind?: ConstJSImportModuleKind;
}

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
