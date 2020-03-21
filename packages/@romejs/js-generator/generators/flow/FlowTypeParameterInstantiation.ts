/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowTypeParameterInstantiation,
  flowTypeParameterInstantiation,
  AnyFlowPrimary,
  FlowTypeParameter,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowTypeParameterInstantiation(
  generator: Generator,
  node: AnyNode,
) {
  node = node.type === 'FlowTypeParameterDeclaration'
    ? node : flowTypeParameterInstantiation.assert(node);

  generator.token('<');
  generator.printCommaList<AnyFlowPrimary | FlowTypeParameter>(node.params, node);
  generator.token('>');
}
