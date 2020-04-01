/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, word, operator} from '../../tokens';
import {
  FlowDeclareModuleExports,
  flowDeclareModuleExports,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareModuleExports(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = flowDeclareModuleExports.assert(node);

  return [
    word('declare'),
    space,
    word('module'),
    operator('.'),
    word('exports'),
    ...generator.printTypeColon(node.typeAnnotation, node),
  ];
}
