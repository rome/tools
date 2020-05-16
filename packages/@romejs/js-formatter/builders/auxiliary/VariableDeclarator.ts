/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {VariableDeclarator} from '@romejs/js-ast';
import {Token, concat, space} from '../../tokens';
import {printAssignment} from '../utils';

export default function VariableDeclarator(
	builder: Builder,
	node: VariableDeclarator,
): Token {
	if (node.init) {
		return printAssignment(
			builder,
			node,
			node.id,
			concat([space, '=']),
			node.init,
		);
	} else {
		return builder.tokenize(node.id, node);
	}
}
