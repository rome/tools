/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {AnyNode, FlowDeclareOpaqueType} from '@romejs/js-ast';
import FlowOpaqueType from './FlowOpaqueType';

export default function FlowDeclareOpaqueType(
  builder: Builder,
  node: FlowDeclareOpaqueType,
  parent: AnyNode,
): Token {
  if (parent.type === 'ExportLocalDeclaration') {
    return FlowOpaqueType(builder, node);
  } else {
    return concat(['declare', space, FlowOpaqueType(builder, node)]);
  }
}
