/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, word, operator} from '../../tokens';
import {
  FlowDeclareModuleExports,
  flowDeclareModuleExports,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareModuleExports(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = flowDeclareModuleExports.assert(node);

  return [
    word('declare'),
    space,
    word('module'),
    operator('.'),
    word('exports'),
    ...builder.tokenizeTypeColon(node.typeAnnotation, node),
  ];
}
