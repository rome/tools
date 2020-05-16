/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {AwaitExpression} from '@romejs/js-ast';

export default function AwaitExpression(
	builder: Builder,
	node: AwaitExpression,
): Token {
	return concat(['await', space, builder.tokenize(node.argument, node)]);
}
