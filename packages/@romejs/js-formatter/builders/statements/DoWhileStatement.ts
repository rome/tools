/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DoWhileStatement} from '@romejs/js-ast';
import Builder from '../../Builder';
import {
	Token,
	concat,
	group,
	hardline,
	indent,
	softline,
	space,
} from '../../tokens';
import {printClause} from '../utils';

export default function DoWhileStatement(
	builder: Builder,
	node: DoWhileStatement,
): Token {
	return concat([
		group(concat(['do', printClause(builder, node.body, node)])),
		node.body.type === 'BlockStatement' ? space : hardline,
		'while',
		space,
		'(',
		group(
			concat([
				indent(concat([softline, builder.tokenize(node.test, node)])),
				softline,
			]),
		),
		')',
		';',
	]);
}
