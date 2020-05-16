/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AssignmentAssignmentPattern,
	BindingAssignmentPattern,
} from '@romejs/js-ast';
import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';

export default function BindingAssignmentPattern(
	builder: Builder,
	node: AssignmentAssignmentPattern | BindingAssignmentPattern,
): Token {
	return concat([
		builder.tokenize(node.left, node),
		space,
		'=',
		space,
		builder.tokenize(node.right, node),
	]);
}
