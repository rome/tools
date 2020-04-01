/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {CatchClause, catchClause, AnyNode} from '@romejs/js-ast';
import {word, space, operator} from '@romejs/js-generator/tokens';

export default function CatchClause(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = catchClause.assert(node);

  return [
    word('catch'),
    space,
    operator('('),
    ...generator.print(node.param, node),
    operator(')'),
    space,
    ...generator.print(node.body, node),
  ];
}
