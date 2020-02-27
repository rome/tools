/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  ArrowFunctionExpression,
  AnyBindingPattern,
  arrowFunctionExpression,
  AnyNode,
} from '@romejs/js-ast';

export default function ArrowFunctionExpression(
  generator: Generator,
  node: AnyNode,
) {
  node = arrowFunctionExpression.assert(node);

  if (node.head.async === true) {
    generator.word('async');
    generator.space();
  }

  const firstParam = node.head.params[0];

  if (
    node.head.params.length === 1 &&
    firstParam !== undefined &&
    firstParam.type === 'BindingIdentifier' &&
    node.head.rest === undefined &&
    !hasTypes(generator, node, firstParam)
  ) {
    generator.print(firstParam, node);
  } else {
    generator.print(node.head, node);
  }

  generator.space();
  generator.token('=>');
  generator.space();

  generator.print(node.body, node);
}

function hasTypes(
  generator: Generator,
  node: ArrowFunctionExpression,
  param: AnyBindingPattern,
): boolean {
  if (generator.options.typeAnnotations) {
    if (
      node.head.typeParameters !== undefined ||
      node.head.returnType !== undefined
    ) {
      return true;
    }

    if (param.meta !== undefined) {
      if (
        param.meta.typeAnnotation !== undefined ||
        param.meta.optional === true
      ) {
        return true;
      }
    }

    return (
      param.trailingComments !== undefined && param.trailingComments.length > 0
    );
  } else {
    return false;
  }
}
