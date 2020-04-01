/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSExportAssignment, tsExportAssignment, AnyNode} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens, space, operator, word} from '../../tokens';

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
    ...builder.print(node.expression, node),
    operator(';'),
  ];
}
