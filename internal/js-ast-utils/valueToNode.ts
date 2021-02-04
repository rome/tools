/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	JSArrayExpression,
	JSBigIntLiteral,
	JSBooleanLiteral,
	JSNullLiteral,
	JSNumericLiteral,
	JSObjectExpression,
	JSObjectProperty,
	JSReferenceIdentifier,
	JSStringLiteral,
	jsArrayExpression,
	jsBigIntLiteral,
	jsBooleanLiteral,
	jsNullLiteral,
	jsNumericLiteral,
	jsObjectExpression,
	jsObjectProperty,
	jsReferenceIdentifier,
	jsStaticPropertyKey,
	jsStringLiteral,
} from "@internal/ast";
import {createPropertyKey} from "./createPropertyKey";
import {UnknownObject} from "@internal/typescript-helpers";

export function valueToNode(
	value: unknown,
	ancestry: unknown[] = [],
):
	| JSStringLiteral
	| JSBigIntLiteral
	| JSBooleanLiteral
	| JSNumericLiteral
	| JSObjectExpression
	| JSNullLiteral
	| JSReferenceIdentifier
	| JSArrayExpression
	| JSBigIntLiteral {
	if (ancestry.includes(value)) {
		throw new Error("Recursion detected");
	}

	switch (typeof value) {
		case "string":
			return jsStringLiteral.quick(value);

		case "boolean":
			return jsBooleanLiteral.quick(value);

		case "number":
			return jsNumericLiteral.quick(value, {raw: String(value)});

		case "bigint":
			return jsBigIntLiteral.quick(String(value));

		case "undefined":
			return jsReferenceIdentifier.quick("undefined");

		case "object": {
			if (value === null) {
				return jsNullLiteral.create({});
			}

			const subAncestry = [...ancestry, value];

			if (Array.isArray(value)) {
				return jsArrayExpression.quick(
					value.map((elem) => valueToNode(elem, subAncestry)),
				);
			}

			const obj = value as UnknownObject;
			const props: JSObjectProperty[] = [];

			for (let key in obj) {
				props.push(
					jsObjectProperty.create({
						key: jsStaticPropertyKey.create({
							value: createPropertyKey(key),
						}),
						value: valueToNode(obj[key], subAncestry),
					}),
				);
			}

			return jsObjectExpression.quick(props);
		}

		default:
			throw new Error("Do not know how to turn this value into a literal");
	}
}
