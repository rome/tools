/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowDeclareVariable,
  flowDeclareVariable,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareVariable(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node = flowDeclareVariable.assert(node);

  if (parent.type !== 'ExportNamedDeclaration') {
    generator.word('declare');
    generator.space();
  }
  generator.word('var');
  generator.space();

  const {id} = node;
  generator.print(id, node);
  if (id.meta !== undefined) {
    generator.print(id.meta.typeAnnotation, node);
  }

  generator.semicolon();
}
