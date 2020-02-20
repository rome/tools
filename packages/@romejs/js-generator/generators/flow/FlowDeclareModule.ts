/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {FlowDeclareModule, flowDeclareModule, AnyNode} from '@romejs/js-ast';

export default function FlowDeclareModule(generator: Generator, node: AnyNode) {
  node = flowDeclareModule.assert(node);

  generator.word('declare');
  generator.space();
  generator.word('module');
  generator.space();
  generator.print(node.id, node);
  generator.space();
  generator.print(node.body, node);
}
