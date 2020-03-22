/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {BigIntLiteral, bigIntLiteral, AnyNode} from '@romejs/js-ast';

export default function BigIntLiteral(generator: Generator, node: AnyNode) {
  node = bigIntLiteral.assert(node);

  generator.word(`${node.value}n`);
}
