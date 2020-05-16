/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSTypeQuery} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Token, concat, space} from '../../tokens';

export default function TSTypeQuery(builder: Builder, node: TSTypeQuery): Token {
	return concat(['typeof', space, builder.tokenize(node.exprName, node)]);
}
