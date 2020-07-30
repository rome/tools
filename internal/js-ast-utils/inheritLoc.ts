/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {SourceLocation} from "@internal/parser-core";

export function inheritLoc(
	node: AnyNode,
	name?: string,
): undefined | SourceLocation {
	const {loc} = node;
	if (loc === undefined) {
		return undefined;
	}

	// Inherit new name if specified
	if (name !== undefined) {
		return {
			...loc,
			identifierName: name,
		};
	}

	// Don't infer a name if it already has one
	if (loc.identifierName !== undefined) {
		return loc;
	}

	// If this location has no identifierName and we're an JSIdentifier then inherit it

	// TODO maybe handle other identifier types? JSXIdentifier etc?
	if (node.type === "JSIdentifier") {
		return {
			...loc,
			identifierName: node.name,
		};
	}

	return loc;
}
