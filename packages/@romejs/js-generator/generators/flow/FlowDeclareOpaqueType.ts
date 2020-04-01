/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, word, space} from '../../tokens';
import {
  FlowDeclareOpaqueType,
  flowDeclareOpaqueType,
  AnyNode,
} from '@romejs/js-ast';
import FlowOpaqueType from './FlowOpaqueType';

export default function FlowDeclareOpaqueType(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
): Tokens {
  node = flowDeclareOpaqueType.assert(node);

  if (parent.type === 'ExportLocalDeclaration') {
    return FlowOpaqueType(generator, node);
  } else {
    return [word('declare'), space, ...FlowOpaqueType(generator, node)];
  }
}
