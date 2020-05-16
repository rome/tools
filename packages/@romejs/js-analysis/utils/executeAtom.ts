/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../scopes';
import T from '../types/T';
import {AnyNode} from '@romejs/js-ast';
import NumericLiteralT from '../types/NumericLiteralT';
import StringLiteralT from '../types/StringLiteralT';
import GetPropT from '../types/GetPropT';

export default function executeAtom(
	leftNode: AnyNode,
	rightType: T,
	scope: Scope,
) {
	switch (leftNode.type) {
		case 'BindingIdentifier': {
			scope.addBinding(leftNode.name, rightType);
			break;
		}

		case 'BindingObjectPattern': {
			for (const prop of leftNode.properties) {
				executeAtom(prop, rightType, scope);
			}
			break;
		}

		case 'BindingObjectPatternProperty': {
			const {key} = leftNode;
			if (key.type === 'ComputedPropertyKey' || key.value.type !== 'Identifier') {
				throw new Error('unimplemented');
			}

			const propKey = new StringLiteralT(scope, key, key.value.name);
			const getProp = new GetPropT(scope, leftNode, rightType, propKey);
			executeAtom(leftNode.value, getProp, scope);
			break;
		}

		case 'BindingArrayPattern': {
			for (let i = 0; i < leftNode.elements.length; i++) {
				const elem = leftNode.elements[i];
				if (elem === undefined) {
					continue;
				}

				const propKey = new NumericLiteralT(scope, elem, i);
				const getProp = new GetPropT(scope, leftNode, rightType, propKey);
				executeAtom(elem, getProp, scope);
			}
			break;
		}

		case 'BindingAssignmentPattern': {
			executeAtom(leftNode.left, rightType, scope);
			break;
		}
	}
}
