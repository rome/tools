/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {AnyNode, FlowFunctionTypeAnnotation} from '@romejs/js-ast';
import {printCommaList} from '../utils';

export default function FlowFunctionTypeAnnotation(
  builder: Builder,
  node: FlowFunctionTypeAnnotation,
  parent: AnyNode,
): Token {
  const tokens: Array<Token> = [
    builder.tokenize(node.typeParameters, node),
    '(',
    printCommaList(builder, node.params, node),
    ')',
  ];

  // this node type is overloaded, not sure why but it makes it EXTREMELY annoying
  if (
    parent.type === 'FlowObjectTypeCallProperty' ||
    parent.type === 'FlowDeclareFunction'
  ) {
    tokens.push(':');
  } else {
    tokens.push(space);
    tokens.push('=>');
  }

  return concat([concat(tokens), space, builder.tokenize(node.returnType, node)]);
}
