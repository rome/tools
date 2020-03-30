/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {TransformExitResult} from '@romejs/js-compiler/types';
import {
  tsTypeReference,
  referenceIdentifier,
  tsTypeParameterInstantiation,
  flowGenericTypeAnnotation,
  flowTypeParameterInstantiation,
} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'noShorthandArrayType',
  enter(path: Path): TransformExitResult {
    const {node, context} = path;

    if (node.type === 'TSArrayType') {
      context.addNodeDiagnostic(node, descriptions.LINT.NO_SHORTHAND_ARRAY_TYPE);

      return tsTypeReference.create({
        typeName: referenceIdentifier.quick('Array'),
        typeParameters: tsTypeParameterInstantiation.create({
          params: [node.elementType],
        }),
      });
    }

    if (node.type === 'FlowArrayTypeAnnotation') {
      context.addNodeDiagnostic(node, descriptions.LINT.NO_SHORTHAND_ARRAY_TYPE);

      return flowGenericTypeAnnotation.create({
        id: referenceIdentifier.quick('Array'),
        typeParameters: flowTypeParameterInstantiation.create({
          params: [node.elementType],
        }),
      });
    }

    return node;
  },
};
