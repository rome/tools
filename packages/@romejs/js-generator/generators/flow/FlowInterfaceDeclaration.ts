/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowInterfaceDeclaration,
  flowInterfaceDeclaration,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowInterfaceDeclaration(
  generator: Generator,
  node: AnyNode,
) {
  node =
    node.type === 'FlowDeclareInterface'
      ? node
      : flowInterfaceDeclaration.assert(node);

  generator.word('interface');
  generator.space();
  _interfaceish(generator, node);
}

export function _interfaceish(generator: Generator, node: AnyNode) {
  node =
    node.type === 'FlowDeclareInterface' || node.type === 'FlowDeclareClass'
      ? node
      : flowInterfaceDeclaration.assert(node);

  generator.print(node.id, node);
  generator.print(node.typeParameters, node);

  if (node.extends.length > 0) {
    generator.space();
    generator.word('extends');
    generator.space();
    generator.printCommaList(node.extends, node);
  }

  if (node.mixins.length > 0) {
    generator.space();
    generator.word('mixins');
    generator.space();
    generator.printCommaList(node.mixins, node);
  }

  generator.space();
  generator.print(node.body, node);
}
