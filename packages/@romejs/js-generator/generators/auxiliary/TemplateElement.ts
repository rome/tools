/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  AnyNode,
  TemplateElement,
  templateLiteral,
  templateElement,
} from '@romejs/js-ast';

export default function TemplateElement(
  generator: Generator,
  node: AnyNode,
  parent: AnyNode,
) {
  node = templateElement.assert(node);
  parent = templateLiteral.assert(parent);

  const isFirst = parent.quasis[0] === node;
  const isLast = parent.quasis[parent.quasis.length - 1] === node;

  const value = (isFirst ? '`' : '}') + node.raw + (isLast ? '`' : '${');

  generator.token(value);
}
