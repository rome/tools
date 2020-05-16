/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {ExportNamespaceSpecifier} from '@romejs/js-ast';

export default function ExportNamespaceSpecifier(
	builder: Builder,
	node: ExportNamespaceSpecifier,
): Token {
	return concat(['*', space, 'as', space, builder.tokenize(node.exported, node)]);
}
