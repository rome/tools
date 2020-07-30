/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {
	jsReferenceIdentifier,
	tsTypeParameterInstantiation,
	tsTypeReference,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

export default createVisitor({
	name: "js/noShorthandArrayType",
	enter(path) {
		const {node} = path;

		if (node.type === "TSArrayType") {
			return path.addFixableDiagnostic(
				{
					fixed: signals.replace(
						tsTypeReference.create({
							typeName: jsReferenceIdentifier.quick("Array"),
							typeParameters: tsTypeParameterInstantiation.create({
								params: [node.elementType],
							}),
						}),
					),
				},
				descriptions.LINT.JS_NO_SHORTHAND_ARRAY_TYPE,
			);
		}

		return signals.retain;
	},
});
