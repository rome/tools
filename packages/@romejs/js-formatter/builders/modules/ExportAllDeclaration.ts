/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, concat, operator, space, word} from '../../tokens';
import {AnyNode, exportAllDeclaration} from '@romejs/js-ast';

export default function ExportAllDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = exportAllDeclaration.assert(node);

  return [
    word('export'),
    space,
    operator('*'),
    space,
    word('from'),
    space,
    concat(builder.tokenize(node.source, node)),
    operator(';'),
  ];
}
