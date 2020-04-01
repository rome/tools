/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {BigIntLiteral, bigIntLiteral, AnyNode} from '@romejs/js-ast';
import {word} from '@romejs/js-generator/tokens';

export default function BigIntLiteral(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = bigIntLiteral.assert(node);
  return [word(`${node.value}n`)];
}
