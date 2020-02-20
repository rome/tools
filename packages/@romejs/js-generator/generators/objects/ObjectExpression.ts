/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {isMultiLine} from '../../node/index';
import {ObjectExpression, objectExpression, AnyNode} from '@romejs/js-ast';

export default function ObjectExpression(generator: Generator, node: AnyNode) {
  node =
    node.type === 'BindingObjectPattern' ||
    node.type === 'AssignmentObjectPattern'
      ? node
      : objectExpression.assert(node);

  const props = node.properties;

  generator.token('{');
  generator.printInnerComments(node);

  if (props.length > 0) {
    generator.printCommaList<typeof props[number]>(props, node, {
      indent: true,
      statement: isMultiLine(node),
    });
  }

  if (
    (node.type === 'BindingObjectPattern' ||
      node.type === 'AssignmentObjectPattern') &&
    node.rest !== undefined
  ) {
    if (props.length > 0) {
      generator.token(',');
      generator.space();
    }

    generator.token('...');
    generator.print(node.rest, node);
  }

  generator.token('}');
}
