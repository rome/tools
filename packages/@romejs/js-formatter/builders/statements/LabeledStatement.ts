/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, space, concat} from '../../tokens';
import {labeledStatement, AnyNode} from '@romejs/js-ast';

export default function LabeledStatement(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = labeledStatement.assert(node);

  return [
    concat(builder.tokenize(node.label, node)),
    operator(':'),
    space,
    concat(builder.tokenize(node.body, node)),
  ];
}
