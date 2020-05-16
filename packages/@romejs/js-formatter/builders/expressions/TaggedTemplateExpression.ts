/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat} from '../../tokens';
import {TaggedTemplateExpression} from '@romejs/js-ast';

export default function TaggedTemplateExpression(
	builder: Builder,
	node: TaggedTemplateExpression,
): Token {
	return concat([
		builder.tokenize(node.tag, node),
		builder.tokenize(node.quasi, node),
	]);
}
