/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {ImportNamespaceSpecifier} from '@romejs/js-ast';

export default function ImportNamespaceSpecifier(
  builder: Builder,
  node: ImportNamespaceSpecifier,
): Token {
  return concat([
    '*',
    space,
    'as',
    space,
    builder.tokenize(node.local.name, node),
  ]);
}
