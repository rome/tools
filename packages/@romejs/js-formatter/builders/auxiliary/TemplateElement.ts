/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {AnyNode, templateLiteral, templateElement} from '@romejs/js-ast';
import {operator} from '@romejs/js-formatter/tokens';

export default function TemplateElement(
  builder: Builder,
  node: AnyNode,
  parent: AnyNode,
) {
  node = templateElement.assert(node);
  parent = templateLiteral.assert(parent);

  const isFirst = parent.quasis[0] === node;
  const isLast = parent.quasis[parent.quasis.length - 1] === node;

  const value = (isFirst ? '`' : '}') + node.raw + (isLast ? '`' : '${');
  return [operator(value)];
}
