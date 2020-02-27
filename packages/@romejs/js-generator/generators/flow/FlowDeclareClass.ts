/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {FlowDeclareClass, flowDeclareClass, AnyNode} from '@romejs/js-ast';
import {_interfaceish} from './FlowInterfaceDeclaration';
export default function FlowDeclareClass(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node = flowDeclareClass.assert(node);

  if (parent.type !== 'ExportNamedDeclaration') {
    generator.word('declare');
    generator.space();
  }
  generator.word('class');
  generator.space();
  _interfaceish(generator, node);
}
