/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {AssignmentExpression} from '@romejs/js-ast';
import {printAssignment} from '../utils';

export default function AssignmentExpression(
	builder: Builder,
	node: AssignmentExpression,
): Token {
	return printAssignment(
		builder,
		node,
		node.left,
		concat([space, node.operator]),
		node.right,
	);
}
