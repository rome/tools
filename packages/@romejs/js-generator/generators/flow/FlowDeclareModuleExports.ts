/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowDeclareModuleExports,
  flowDeclareModuleExports,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareModuleExports(
  generator: Generator,
  node: AnyNode,
) {
  node = flowDeclareModuleExports.assert(node);

  generator.word('declare');
  generator.space();
  generator.word('module');
  generator.token('.');
  generator.word('exports');
  generator.printTypeColon(node.typeAnnotation, node);
}
