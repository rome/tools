/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ConditionalExpression} from '@romejs/js-ast';
import Builder from '../../Builder';
import {Token, concat, group, indent, lineOrSpace, space} from '../../tokens';

export default function ConditionalExpression(
  builder: Builder,
  node: ConditionalExpression,
): Token {
  return printConditionalExpression(
    builder.tokenize(node.test, node),
    builder.tokenize(node.consequent, node),
    builder.tokenize(node.alternate, node),
  );
}

export function printConditionalExpression(
  test: Token,
  consequent: Token,
  alternate: Token,
): Token {
  return group(
    concat([
      test,
      indent(concat([lineOrSpace, '?', space, indent(consequent)])),
      indent(concat([lineOrSpace, ':', space, indent(alternate)])),
    ]),
  );
}
