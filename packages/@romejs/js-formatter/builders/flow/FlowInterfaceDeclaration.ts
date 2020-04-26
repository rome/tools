/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {
  FlowDeclareClass,
  FlowDeclareInterface,
  FlowInterfaceDeclaration,
} from '@romejs/js-ast';
import {printCommaList} from '../utils';

export default function FlowInterfaceDeclaration(
  builder: Builder,
  node: FlowDeclareInterface | FlowInterfaceDeclaration,
): Token {
  return concat(['interface', space, _interfaceish(builder, node)]);
}

export function _interfaceish(
  builder: Builder,
  node: FlowDeclareInterface | FlowDeclareClass | FlowInterfaceDeclaration,
): Token {
  let tokens: Array<Token> = [
    builder.tokenize(node.id, node),
    builder.tokenize(node.typeParameters, node),
  ];

  if (node.extends.length > 0) {
    tokens.push(
      space,
      'extends',
      space,
      printCommaList(builder, node.extends, node),
    );
  }

  if (node.mixins.length > 0) {
    tokens.push(
      space,
      'mixins',
      space,
      printCommaList(builder, node.mixins, node),
    );
  }

  return concat([concat(tokens), space, builder.tokenize(node.body, node)]);
}
