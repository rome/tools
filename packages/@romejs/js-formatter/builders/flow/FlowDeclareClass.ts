/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, word, space} from '../../tokens';
import {FlowDeclareClass, flowDeclareClass, AnyNode} from '@romejs/js-ast';
import {_interfaceish} from './FlowInterfaceDeclaration';

export default function FlowDeclareClass(
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
): Tokens {
  node = flowDeclareClass.assert(node);

  const tokens: Tokens = [];
  if (parent.type !== 'ExportLocalDeclaration') {
    tokens.push(word('declare'));
    tokens.push(space);
  }

  return [...tokens, word('class'), space, ..._interfaceish(builder, node)];
}
