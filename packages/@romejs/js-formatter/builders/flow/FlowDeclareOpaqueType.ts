/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, word} from '../../tokens';
import {
  AnyNode,
  FlowDeclareOpaqueType,
  flowDeclareOpaqueType,
} from '@romejs/js-ast';
import FlowOpaqueType from './FlowOpaqueType';

export default function FlowDeclareOpaqueType(
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
): Tokens {
  node = flowDeclareOpaqueType.assert(node);

  if (parent.type === 'ExportLocalDeclaration') {
    return FlowOpaqueType(builder, node);
  } else {
    return [word('declare'), space, ...FlowOpaqueType(builder, node)];
  }
}
