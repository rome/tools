/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {FlowDeclareModuleExports} from '@romejs/js-ast';

export default function FlowDeclareModuleExports(
  builder: Builder,
  node: FlowDeclareModuleExports,
): Token {
  return concat([
    'declare',
    space,
    'module',
    '.',
    'exports',
    ':',
    space,
    builder.tokenize(node.typeAnnotation, node),
  ]);
}
