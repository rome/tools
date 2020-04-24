/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, operator, space, word} from '../../tokens';
import {
  AnyNode,
  FlowDeclareModuleExports,
  flowDeclareModuleExports,
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
