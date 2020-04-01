/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, operator} from '../../tokens';
import {ImportCall, importCall, AnyNode} from '@romejs/js-ast';

export default function ImportCall(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = importCall.assert(node);

  return [
    word('import'),
    operator('('),
    ...generator.print(node.argument, node),
    operator(')'),
  ];
}
