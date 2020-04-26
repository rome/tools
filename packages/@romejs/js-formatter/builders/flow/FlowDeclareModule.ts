/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowDeclareModule} from '@romejs/js-ast';

export default function FlowDeclareModule(
  builder: Builder,
  node: FlowDeclareModule,
): Token {
  return concat([
    'declare',
    space,
    'module',
    space,
    builder.tokenize(node.id, node),
    space,
    builder.tokenize(node.body, node),
  ]);
}
