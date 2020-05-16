/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSAsExpression} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token, concat, space} from '../../tokens';

export default function TSAsExpression(
	builder: Builder,
	node: TSAsExpression,
): Token {
	if (builder.options.typeAnnotations) {
		return concat([
			builder.tokenize(node.expression, node),
			space,
			'as',
			space,
			builder.tokenize(node.typeAnnotation, node),
		]);
	} else {
		return builder.tokenize(node.expression, node);
	}
}
