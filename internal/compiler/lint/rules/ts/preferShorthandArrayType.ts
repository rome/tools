/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createLintVisitor, signals} from "@internal/compiler";
import {
	AnyNode,
	AnyTSPrimary,
	TSTypeParameterInstantiation,
	TSTypeReference,
	tsArrayType,
	tsUnionTypeAnnotation,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

function isArrayReference(node: AnyNode): node is TSTypeReference {
	return (
		node.type === "TSTypeReference" &&
		node.typeName.type === "JSReferenceIdentifier" &&
		node.typeName.name === "Array"
	);
}

function convertToArrayType(
	typeParameters: TSTypeParameterInstantiation,
): AnyTSPrimary | undefined {
	if (typeParameters.params.length > 0) {
		const arrayTypes: AnyTSPrimary[] = [];

		typeParameters.params.forEach((param) => {
			if (
				param.type !== "TSUnionTypeAnnotation" &&
				param.type !== "TSTypeOperator"
			) {
				const elementType =
					isArrayReference(param) && param.typeParameters
						? convertToArrayType(param.typeParameters)
						: param;
				if (elementType) {
					arrayTypes.push(
						tsArrayType.create({
							elementType,
						}),
					);
				}
			}
		});
		if (arrayTypes.length > 0) {
			return tsUnionTypeAnnotation.create({
				types: arrayTypes,
			});
		}

		return undefined;
	}
	return tsArrayType.create({
		elementType: typeParameters.params[0],
	});
}

export default createLintVisitor({
	name: "ts/preferShorthandArrayType",
	enter(path) {
		const {node} = path;
		if (isArrayReference(node) && node.typeParameters) {
			let toReplace: AnyNode | undefined = convertToArrayType(
				node.typeParameters,
			);
			if (toReplace) {
				return path.addFixableDiagnostic(
					{
						fixed: signals.replace(toReplace),
					},
					descriptions.LINT.TS_PREFER_SHORTHAND_ARRAY_TYPE,
				);
			}
		}

		return signals.retain;
	},
});
