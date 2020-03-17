/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowDeclareFunction,
  flowDeclareFunction,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareFunction(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node = flowDeclareFunction.assert(node);

  if (parent.type !== 'ExportLocalDeclaration') {
    generator.word('declare');
    generator.space();
  }
  generator.word('function');
  generator.space();
  generator.print(node.id, node);
  generator.semicolon();
}
