/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, operator, space} from '../../tokens';
import {LabeledStatement, labeledStatement, AnyNode} from '@romejs/js-ast';

export default function LabeledStatement(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = labeledStatement.assert(node);

  return [
    ...generator.print(node.label, node),
    operator(':'),
    space,
    ...generator.print(node.body, node),
  ];
}
