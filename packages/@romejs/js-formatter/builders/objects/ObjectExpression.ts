/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AssignmentObjectPattern,
  BindingObjectPattern,
  ObjectExpression,
} from '@romejs/js-ast';
import Builder from '../../Builder';
import {
  Token,
  concat,
  group,
  hardline,
  ifBreak,
  indent,
  lineOrSpace,
  softline,
} from '../../tokens';
import {hasInnerComments} from '../comments';
import {printCommaList} from '../utils';

export default function ObjectExpression(
  builder: Builder,
  node: ObjectExpression | AssignmentObjectPattern | BindingObjectPattern,
): Token {
  if (hasInnerComments(node)) {
    return concat([
      '{',
      builder.tokenizeInnerComments(node, true),
      hardline,
      '}',
    ]);
  }

  const props = node.properties;

  const tokens: Array<Token> = [printCommaList(builder, props, node)];

  if (
    (node.type === 'BindingObjectPattern' ||
    node.type === 'AssignmentObjectPattern') &&
    node.rest !== undefined
  ) {
    if (props.length > 0) {
      tokens.push(',', lineOrSpace);
    }

    tokens.push('...', builder.tokenize(node.rest, node));
  } else {
    // Add trailing comma
    tokens.push(ifBreak(','));
  }

  // If the first property is not one the same line as the opening brace,
  // the object is printed on multiple lines.
  const shouldBreak =
    node.loc !== undefined &&
    props.length > 0 &&
    props[0].loc !== undefined &&
    props[0].loc.start.line !== node.loc.start.line;

  return group(
    concat(['{', indent(concat([softline, concat(tokens)])), softline, '}']),
    shouldBreak,
  );
}
