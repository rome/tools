/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, TSExportAssignment, tsExportAssignment} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, operator, space, word} from '../../tokens';

export default function TSExportAssignment(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = tsExportAssignment.assert(node);

  return [
    word('export'),
    space,
    operator('='),
    space,
    ...builder.tokenize(node.expression, node),
    operator(';'),
  ];
}
