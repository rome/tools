/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {LabeledStatement} from '@romejs/js-ast';

export default function LabeledStatement(
	builder: Builder,
	node: LabeledStatement,
): Token {
	return concat([
		builder.tokenize(node.label, node),
		':',
		node.body.type === 'EmptyStatement'
			? ';'
			: concat([space, builder.tokenize(node.body, node)]),
	]);
}
