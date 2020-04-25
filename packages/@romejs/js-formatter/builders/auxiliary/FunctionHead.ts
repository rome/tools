/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {FunctionHead} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {printBindingPatternParams} from '../utils';
import {Token, concat, group, space} from '../../tokens';

export default function FunctionHead(
  builder: Builder,
  node: FunctionHead,
): Token {
  const tokens: Array<Token> = [];

  if (builder.options.typeAnnotations && node.typeParameters) {
    tokens.push(builder.tokenize(node.typeParameters, node));
  }

  const printedParameters = printBindingPatternParams(
    builder,
    node,
    node.params,
    node.rest,
  );

  let printedReturnType: Token = '';
  if (builder.options.typeAnnotations) {
    if (node.returnType || node.predicate) {
      const tokens: Array<Token> = [':'];

      if (node.returnType) {
        tokens.push(space, builder.tokenize(node.returnType, node));
      }

      if (node.predicate) {
        tokens.push(space, builder.tokenize(node.predicate, node));
      }

      printedReturnType = concat(tokens);
    }
  }

  tokens.push(group(concat([printedParameters, printedReturnType])));

  return concat(tokens);
}
