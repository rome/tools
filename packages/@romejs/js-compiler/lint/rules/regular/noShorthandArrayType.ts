/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {TransformExitResult} from '@romejs/js-compiler/types';
import {
  referenceIdentifier,
  tsTypeParameterInstantiation,
  tsTypeReference,
} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

export default {
  name: 'noShorthandArrayType',
  enter(path: Path): TransformExitResult {
    const {node, context} = path;

    if (node.type === 'TSArrayType') {
      return context.addFixableDiagnostic(
        {
          old: node,
          fixed: tsTypeReference.create({
            typeName: referenceIdentifier.quick('Array'),
            typeParameters: tsTypeParameterInstantiation.create({
              params: [node.elementType],
            }),
          }),
        },
        descriptions.LINT.NO_SHORTHAND_ARRAY_TYPE,
      );
    }

    return node;
  },
};
