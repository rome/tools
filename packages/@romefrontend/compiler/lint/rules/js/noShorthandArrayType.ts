/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from "@romefrontend/compiler";
import {TransformExitResult} from "@romefrontend/compiler/types";
import {
	jsReferenceIdentifier,
	tsTypeParameterInstantiation,
	tsTypeReference,
} from "@romefrontend/ast";
import {descriptions} from "@romefrontend/diagnostics";

export default {
	name: "noShorthandArrayType",
	enter(path: Path): TransformExitResult {
		const {node, context} = path;

		if (node.type === "TSArrayType") {
			return context.addFixableDiagnostic(
				{
					old: node,
					fixed: tsTypeReference.create({
						typeName: jsReferenceIdentifier.quick("Array"),
						typeParameters: tsTypeParameterInstantiation.create({
							params: [node.elementType],
						}),
					}),
				},
				descriptions.LINT.JS_NO_SHORTHAND_ARRAY_TYPE,
			);
		}

		return node;
	},
};
