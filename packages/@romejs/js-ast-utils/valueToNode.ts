/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ArrayExpression,
	BooleanLiteral,
	NullLiteral,
	NumericLiteral,
	ObjectExpression,
	ObjectProperty,
	ReferenceIdentifier,
	StringLiteral,
	arrayExpression,
	booleanLiteral,
	nullLiteral,
	numericLiteral,
	objectExpression,
	objectProperty,
	referenceIdentifier,
	staticPropertyKey,
	stringLiteral,
} from '@romejs/js-ast';
import createPropertyKey from './createPropertyKey';
import {UnknownObject} from '@romejs/typescript-helpers';

export default function valueToNode(
	value: unknown,
	ancestry: Array<unknown> = [],
):
	 | StringLiteral
	| BooleanLiteral
	| NumericLiteral
	| ObjectExpression
	| NullLiteral
	| ReferenceIdentifier
	| ArrayExpression {
	if (ancestry.includes(value)) {
		throw new Error('Recursion detected');
	}

	switch (typeof value) {
		case 'string':
			return stringLiteral.quick(value);

		case 'boolean':
			return booleanLiteral.quick(value);

		case 'number':
			return numericLiteral.quick(value);

		case 'undefined':
			return referenceIdentifier.quick('undefined');

		case 'object': {
			if (value === null) {
				return nullLiteral.create({});
			}

			const subAncestry = [...ancestry, value];

			if (Array.isArray(value)) {
				return arrayExpression.quick(
					value.map((elem) => valueToNode(elem, subAncestry)),
				);
			}

			const obj = (value as UnknownObject);
			const props: Array<ObjectProperty> = [];

			for (let key in obj) {
				props.push(
					objectProperty.create({
						key: staticPropertyKey.create({
							value: createPropertyKey(key),
						}),
						value: valueToNode(obj[key], subAncestry),
					}),
				);
			}

			return objectExpression.quick(props);
		}

		default:
			throw new Error('Do not know how to turn this value into a literal');
	}
}
