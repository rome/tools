/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {ForOfStatement} from '@romejs/js-ast';
import {Token, concat, group, space} from '../../tokens';
import {printClause} from '../utils';

export default function ForOfStatement(
	builder: Builder,
	node: ForOfStatement,
): Token {
	return group(
		concat([
			'for',
			node.await ? concat([space, 'await']) : '',
			space,
			'(',
			builder.tokenize(node.left, node),
			space,
			'of',
			space,
			builder.tokenize(node.right, node),
			')',
			printClause(builder, node.body, node),
		]),
	);
}
