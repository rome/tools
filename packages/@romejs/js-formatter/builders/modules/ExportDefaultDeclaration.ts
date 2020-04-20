/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space, concat} from '../../tokens';
import {exportDefaultDeclaration, AnyNode} from '@romejs/js-ast';
import {_ExportDeclaration} from './ExportLocalDeclaration';

export default function ExportDefaultDeclaration(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = exportDefaultDeclaration.assert(node);

  return [
    word('export'),
    word('default'),
    space,
    concat(_ExportDeclaration(builder, node)),
  ];
}
