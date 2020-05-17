/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ConstImportModuleKind,
	ImportDefaultSpecifier,
	ImportNamespaceSpecifier,
	ImportSpecifier,
	JSNodeBase,
	StringLiteral,
} from "../index";
import {createBuilder} from "../utils";

export type AnyImportSpecifier =
	| ImportDefaultSpecifier
	| ImportNamespaceSpecifier
	| ImportSpecifier;

export type ImportDeclaration = JSNodeBase & {
	type: "ImportDeclaration";
	defaultSpecifier?: ImportDefaultSpecifier;
	namespaceSpecifier?: ImportNamespaceSpecifier;
	namedSpecifiers: Array<ImportSpecifier>;
	source: StringLiteral;
	importKind?: ConstImportModuleKind;
};

export const importDeclaration = createBuilder<ImportDeclaration>(
	"ImportDeclaration",
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
