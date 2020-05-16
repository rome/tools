/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BigIntLiteral} from '@romejs/js-ast';
import Builder from '../../Builder';
import {Token} from '../../tokens';

export default function BigIntLiteral(
	builder: Builder,
	node: BigIntLiteral,
): Token {
	return `${node.value}n`;
}
