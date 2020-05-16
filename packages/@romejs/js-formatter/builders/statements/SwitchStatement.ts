/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

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
import {SwitchStatement} from '@romejs/js-ast';

export default function SwitchStatement(
	builder: Builder,
	node: SwitchStatement,
): Token {
	return concat([
		group(
			concat([
				'switch',
				space,
				'(',
				group(
					concat([
						indent(concat([softline, builder.tokenize(node.discriminant, node)])),
						softline,
					]),
				),
				')',
			]),
		),
		space,
		'{',
		node.cases.length > 0
			? indent(concat([hardline, builder.tokenizeStatementList(node.cases, node)]))
			: '',
		hardline,
		'}',
	]);
}
