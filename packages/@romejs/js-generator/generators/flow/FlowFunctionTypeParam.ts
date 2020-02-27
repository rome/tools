/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowFunctionTypeParam,
  flowFunctionTypeParam,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowFunctionTypeParam(
  generator: Generator,
  node: AnyNode,
) {
  node = flowFunctionTypeParam.assert(node);

  if (node.name) {
    generator.print(node.name, node);
    if (node.meta.optional === true) {
      generator.token('?');
    }
    generator.token(':');
    generator.space();
  }
  generator.print(node.meta.typeAnnotation, node);
}
