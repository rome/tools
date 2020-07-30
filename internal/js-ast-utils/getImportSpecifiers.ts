/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyImportSpecifier, JSImportDeclaration} from "@internal/ast";

export function getImportSpecifiers(
	node: JSImportDeclaration,
): Array<AnyImportSpecifier> {
	let specifiers: Array<AnyImportSpecifier> = [];

	if (node.defaultSpecifier !== undefined) {
		specifiers.push(node.defaultSpecifier);
	}

	if (node.namespaceSpecifier !== undefined) {
		specifiers.push(node.namespaceSpecifier);
	}

	specifiers = specifiers.concat(node.namedSpecifiers);

	return specifiers;
}
