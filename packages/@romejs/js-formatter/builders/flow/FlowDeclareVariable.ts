/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {AnyNode, FlowDeclareVariable} from '@romejs/js-ast';

export default function FlowDeclareVariable(
  builder: Builder,
  node: FlowDeclareVariable,
  parent: AnyNode,
): Token {
  const tokens: Array<Token> = [];

  if (parent.type !== 'ExportLocalDeclaration') {
    tokens.push('declare');
    tokens.push(space);
  }

  tokens.push('var');
  tokens.push(space);

  const {id} = node;

  tokens.push(builder.tokenize(id, node));

  if (id.meta !== undefined) {
    tokens.push(builder.tokenize(id.meta.typeAnnotation, node));
  }

  tokens.push(';');

  return concat(tokens);
}
