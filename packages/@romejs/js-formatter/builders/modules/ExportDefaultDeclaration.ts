/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space} from '../../tokens';
import {
  ExportDefaultDeclaration,
  exportDefaultDeclaration,
  AnyNode,
} from '@romejs/js-ast';
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
    ..._ExportDeclaration(builder, node),
  ];
}
