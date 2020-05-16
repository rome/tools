/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {DoExpression} from '@romejs/js-ast';

export default function DoExpression(
	builder: Builder,
	node: DoExpression,
): Token {
	return concat(['do', space, builder.tokenize(node.body, node)]);
}
