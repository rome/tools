/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {AnyNode, FlowDeclareClass} from '@romejs/js-ast';
import {_interfaceish} from './FlowInterfaceDeclaration';

export default function FlowDeclareClass(
  builder: Builder,
  node: FlowDeclareClass,
  parent: AnyNode,
): Token {
  const tokens: Array<Token> = [];

  if (parent.type !== 'ExportLocalDeclaration') {
    tokens.push('declare');
    tokens.push(space);
  }

  return concat([concat(tokens), 'class', space, _interfaceish(builder, node)]);
}
