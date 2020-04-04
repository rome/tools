/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyArrayPattern,
  ArrayExpression,
  AssignmentArrayPattern,
  BindingArrayPattern,
} from '@romejs/js-ast';
import Builder from '../../Builder';
import {
  Token,
  concat,
  group,
  hardline,
  ifBreak,
  indent,
  join,
  lineOrSpace,
  softline,
} from '../../tokens';
import {hasInnerComments} from '../comments';

export default function ArrayExpression(
  builder: Builder,
  node: ArrayExpression | BindingArrayPattern | AssignmentArrayPattern,
): Token {
  const hasContents = node.elements.length > 0;
  const hasRest =
    (node.type === 'BindingArrayPattern' ||
    node.type === 'AssignmentArrayPattern') &&
    node.rest !== undefined;

  if (!hasContents && !hasRest) {
    if (hasInnerComments(node)) {
      return concat([
        '[',
        builder.tokenizeInnerComments(node, true),
        hardline,
        ']',
      ]);
    } else {
      return '[]';
    }
  }

  const tokens: Array<Token> = [];

  if (hasContents) {
    const elements: Array<Token> = [];

    for (const element of node.elements) {
      if (element === undefined) {
        elements.push('');
      } else {
        elements.push(builder.tokenize(element, node));
      }
    }

    tokens.push(join(concat([',', lineOrSpace]), elements));

    if (hasRest) {
      tokens.push(',', lineOrSpace);
    } else {
      // Add trailing comma
      tokens.push(ifBreak(','));
    }
  }

  if (hasRest) {
    tokens.push('...', builder.tokenize((node as AnyArrayPattern).rest, node));
  }

  return group(
    concat(['[', indent(concat([softline, concat(tokens)])), softline, ']']),
  );
}
