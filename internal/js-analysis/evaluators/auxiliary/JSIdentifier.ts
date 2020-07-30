/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "../../scopes";
import {AnyNode, JSIdentifier, jsIdentifier} from "@internal/ast";
import UndeclaredVarE from "../../types/errors/UndeclaredVarE";
import OpenT from "../../types/OpenT";
import AnyT from "../../types/AnyT";

export default function JSIdentifier(node: AnyNode, scope: Scope) {
	node = jsIdentifier.assert(node);

	const binding = scope.getBinding(node.name);
	if (binding) {
		const type = new OpenT(scope, node);
		type.shouldMatch(binding);
		return type;
	} else {
		switch (node.name) {
			case "React$PropType$Primitive":
			case "React$PropType$ArrayOf":
			case "React$PropType$InstanceOf":
			case "React$PropType$ObjectOf":
			case "React$PropType$OneOf":
			case "React$PropType$OneOfType":
			case "React$PropTypePartial":
			case "React$ElementProps":
			case "React$ElementRef":
			case "$Exact":
			case "Partial":
			case "$Keys":
			case "Object$Assign":
			case "Object$GetPrototypeOf":
			case "Object$SetPrototypeOf":
			case "$CharSet":
			case "Class":
			case "$Compose":
			case "$ComposeReverse":
			case "$Subtype":
			case "Function$Prototype$Apply":
			case "Function$Prototype$Bind":
			case "Function$Prototype$Call":
			case "$Exports":
				return new AnyT(scope, node);

			default:
				return new UndeclaredVarE(scope, node, node.name);
		}
	}
}
